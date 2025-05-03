
#[cfg(test)]
mod type_tests {
    use super::*;

    use crate::types::*;
    use crate::utils::*;

    #[test]
    fn type_value() {
        let mut ctxt = Context::default();
        assert_eq!(ctxt.type_expr(&mut Expr::Unit), Ok(Type::Unit));
        assert_eq!(ctxt.type_expr(&mut Expr::Int(42)), Ok(Type::Int));
    }

    #[test]
    fn make_copy() {
        let mut ctxt = Context::default();
        ctxt.env.insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        let mut e = Expr::Lv(Lval::new("x", 1), Copyable::No);
        assert_eq!(ctxt.type_expr(&mut e), Ok(Type::Int));
        assert_eq!(e, Expr::Lv(Lval::new("x", 1), Copyable::Yes));
    }

    #[test]
    fn keep_move() {
        let mut ctxt = Context::default();
        ctxt.env.insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        let mut e = Expr::Lv(Lval::new("x", 0), Copyable::No);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Ok(Type::TBox(Box::new(Type::Int)))
        );
        assert_eq!(e, Expr::Lv(Lval::new("x", 0), Copyable::No));
        assert_eq!(
            ctxt.env.type_lval(&Lval::new("x", 0)).map(|s| s.tipe),
            Ok(Type::Undefined(Box::new(Type::TBox(Box::new(Type::Int)))))
        );
    }

    #[test]
    fn cannot_copy() {
        let mut ctxt = Context::default();
        ctxt.env.insert("x", Type::Int, Lifetime(1));
        ctxt.env.insert("y", Type::Ref(Lval::new("x", 0), Mutable::Yes), Lifetime(1));
        let mut e = Expr::Lv(Lval::new("x", 0), Copyable::No);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Err(Error::CopyAfterMutBorrow(Lval::new("x", 0)))
        );
    }

    #[test]
    fn cannot_move() {
        let mut ctxt = Context::default();
        ctxt.env.insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        ctxt.env.insert("y", Type::Ref(Lval::new("x", 0), Mutable::No), Lifetime(1));
        let mut e = Expr::Lv(Lval::new("x", 0), Copyable::No);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Err(Error::MoveAfterBorrow(Lval::new("x", 0)))
        );
    }

    #[test]
    fn move_behind_ref() {
        let mut ctxt = Context::default();
        ctxt.env.insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        ctxt.env.insert("y", Type::Ref(Lval::new("x", 0), Mutable::No), Lifetime(1));
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("y", 1), Copyable::No)),
            Err(Error::MoveBehindRef(Lval::new("y", 1)))
        );
    }

    #[test]
    fn invalid_lval() {
        let mut ctxt = Context::default();
        ctxt.env.insert("x", Type::Int, Lifetime(1));
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("x", 1), Copyable::Yes)),
            Err(Error::CannotDeref(Type::Int))
        );
    }

    #[test]
    fn moved_out() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::Int)))),
            Lifetime(1),
        );
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("x", 1), Copyable::No)),
            Ok(Type::TBox(Box::new(Type::Int)))
        );
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("x", 2), Copyable::No)),
            Err(Error::MovedOut(Lval::new("x", 2)))
        );
    }

    #[test]
    fn still_moved_out() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::Int)))),
            Lifetime(1),
        );
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("x", 1), Copyable::No)),
            Ok(Type::TBox(Box::new(Type::Int)))
        );
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("x", 0), Copyable::No)),
            Err(Error::MovedOut(Lval::new("x", 0)))
        );
    }

    #[test]
    fn copied() {
        let mut ctxt = Context::default();
        ctxt.env
            .insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("x", 1), Copyable::Yes)),
            Ok(Type::Int)
        );
        assert_eq!(
            ctxt.type_expr(&mut Expr::Lv(Lval::new("x", 1), Copyable::Yes)),
            Ok(Type::Int)
        );
    }

    #[test]
    fn imm_borrow_ok() {
        let mut ctxt = Context::default();
        ctxt.env
            .insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        ctxt.env.insert(
            "y",
            Type::Ref(Lval::new("x", 0), Mutable::No),
            Lifetime(1),
        );
        let mut e = Expr::Borrow(Lval::new("x", 1), Mutable::No);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Ok(Type::Ref(Lval::new("x", 1), Mutable::No))
        );
    }

    #[test]
    fn imm_borrow_err() {
        let mut ctxt = Context::default();
        ctxt.env
            .insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        ctxt.env.insert(
            "y",
            Type::Ref(Lval::new("x", 1), Mutable::Yes),
            Lifetime(1),
        );
        let mut e = Expr::Borrow(Lval::new("x", 0), Mutable::No);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Err(Error::BorrowAfterMutBorrow(Lval::new("x", 0)))
        );
    }

    #[test]
    fn imm_borrow_err_moved_out() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::Undefined(Box::new(Type::Int)))),
            Lifetime(1),
        );
        let mut e = Expr::Borrow(Lval::new("x", 1), Mutable::No);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Err(Error::MovedOut(Lval::new("x", 1)))
        );
    }

    #[test]
    fn mut_borrow_ok() {
        let mut ctxt = Context::default();
        ctxt.env
            .insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        let mut e = Expr::Borrow(Lval::new("x", 1), Mutable::Yes);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Ok(Type::Ref(Lval::new("x", 1), Mutable::Yes))
        );
    }

    #[test]
    fn mut_borrow_err_already_borrowed() {
        let mut ctxt = Context::default();
        ctxt.env
            .insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        ctxt.env.insert(
            "y",
            Type::Ref(Lval::new("x", 1), Mutable::No),
            Lifetime(1),
        );
        let mut e = Expr::Borrow(Lval::new("x", 0), Mutable::Yes);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Err(Error::MutBorrowAfterBorrow(Lval::new("x", 0)))
        );
    }

    #[test]
    fn mut_borrow_through_ref() {
        let mut ctxt = Context::default();
        ctxt.env
            .insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        ctxt.env.insert(
            "y",
            Type::Ref(Lval::new("x", 1), Mutable::Yes),
            Lifetime(1),
        );
        ctxt.env.insert(
            "z",
            Type::Ref(Lval::new("y", 0), Mutable::Yes),
            Lifetime(1),
        );
        let mut e = Expr::Borrow(Lval::new("z", 2), Mutable::Yes);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Ok(Type::Ref(Lval::new("z", 2), Mutable::Yes))
        );
    }

    #[test]
    fn mut_borrow_err_through_imm_ref() {
        let mut ctxt = Context::default();
        ctxt.env
            .insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        ctxt.env.insert(
            "y",
            Type::Ref(Lval::new("x", 1), Mutable::No),
            Lifetime(1),
        );
        ctxt.env.insert(
            "z",
            Type::Ref(Lval::new("y", 0), Mutable::Yes),
            Lifetime(1),
        );
        let mut e = Expr::Borrow(Lval::new("z", 2), Mutable::Yes);
        assert_eq!(
            ctxt.type_expr(&mut e),  
            Err(Error::MutBorrowBehindImmRef(Lval::new("z", 2)))
        );
    }

    #[test]
    fn mut_borrow_err_moved_out() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::Undefined(Box::new(Type::Int)))),
            Lifetime(1),
        );
        let mut e = Expr::Borrow(Lval::new("x", 1), Mutable::Yes);
        assert_eq!(
            ctxt.type_expr(&mut e),
            Err(Error::MovedOut(Lval::new("x", 1)))
        );
    }

    #[test]
    fn type_box() {
        let mut ctxt = Context::default();
        assert_eq!(
            ctxt.type_expr(&mut Expr::OBox(Box::new(Expr::Int(30)))),
            Ok(Type::TBox(Box::new(Type::Int)))
        );
    }

    #[test]
    fn declare_ok() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::Int)),
            Lifetime::global()
        );
        let mut ctxt_2 = ctxt.clone();
        assert!(ctxt.type_stmt(&mut Stmt::LetMut(
            "y".to_string(),
            Expr::Borrow(Lval::new("x", 1), Mutable::Yes)
        )).is_ok());
        ctxt_2.env.insert(
            "y",
            Type::Ref(Lval::new("x", 1), Mutable::Yes),
            Lifetime::global()
        );
        assert_eq!(ctxt, ctxt_2);
    }

    #[test]
    fn declare_shadow() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::Int)),
            Lifetime::global()
        );
        assert_eq!(
            ctxt.type_stmt(&mut Stmt::LetMut(
                "x".to_string(),
                Expr::Int(30)
            )),
            Err(Error::Shadowing("x".to_string()))
        );
    }

    #[test]
    fn declare_moved_out() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::Undefined(Box::new(Type::Int)))),
            Lifetime::global()
        );
        assert_eq!(
            ctxt.type_stmt(&mut Stmt::LetMut(
                "y".to_string(),
                Expr::Lv(Lval::new("x", 1), Copyable::No)
            )),
            Err(Error::MovedOut(Lval::new("x", 1)))
        );
    }

    #[test]
    fn assign_ok() {
        let mut ctxt = Context::default();
        ctxt.env.insert(
            "x",
            Type::TBox(Box::new(Type::Int)),
            Lifetime::global()
        );
        let ctxt_2 = ctxt.clone();
        assert!(ctxt.type_stmt(&mut Stmt::Assign(
            Lval::new("x", 0),
            Expr::OBox(Box::new(Expr::Int(40)))
        )).is_ok());
        assert!(ctxt.type_stmt(&mut Stmt::Assign(
            Lval::new("x", 1),
            Expr::Int(30)
        )).is_ok());
        assert_eq!(ctxt, ctxt_2);
    }

    // …and so on for the rest of your tests…

}
