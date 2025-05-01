
use crate::Env;
 use crate::Slot;
use crate::Type;
 use crate::Lifetime;
use crate::Lval;

 use crate::Copyable;
use crate::Context;
use crate::Mutable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_var() {
        let mut env = Env::default();
        // Slot::new(Type::Unit, Lifetime(1))
        let slot = Slot { tipe: Type::Unit, lifetime: Lifetime(1) };
        env.insert("x", Type::Unit, Lifetime(1));
        env.insert("y", Type::Int, Lifetime(1));
        assert_eq!(env.type_lval(&Lval::new("x", 0)).unwrap(), slot);
    }

    #[test]
    fn env_lval_box() {
        let mut env = Env::default();
        // Slot::new(Type::Int, Lifetime(3))
        let slot = Slot { tipe: Type::Int, lifetime: Lifetime(3) };
        // Type::TBox(Box::new(Type::Int))
        env.insert("x", Type::TBox(Box::new(Type::Int)), Lifetime(3));
        assert_eq!(env.type_lval(&Lval::new("x", 1)).unwrap(), slot);
    }

    #[test]
    fn env_lval_ref() {
        let mut env = Env::default();
        // Slot::new(Type::Int, Lifetime(1))
        let slot = Slot { tipe: Type::Int, lifetime: Lifetime(1) };
        // Type::Ref(Lval::new("y",1), Mutable::No)
        env.insert("x", Type::Ref(Lval::new("y", 1), Mutable::No), Lifetime(3));
        env.insert("y", Type::TBox(Box::new(Type::Int)), Lifetime(1));
        assert_eq!(env.type_lval(&Lval::new("x", 1)).unwrap(), slot);
    }

    #[test]
    fn env_contained() {
        let mut env = Env::default();
        env.insert(
            "y",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::Int)))),
            Lifetime(1),
        );
        assert_eq!(*env.contained(&String::from("y")).unwrap(), Type::Int);
    }

    #[test]
    fn env_contained_undefined() {
        let mut env = Env::default();
        env.insert(
            "y",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::Undefined(Box::new(Type::Int)))))),
            Lifetime(1),
        );
        assert_eq!(env.contained(&String::from("y")), None);
    }

    #[test]
    fn basic_read_prohibited() {
        let mut env = Env::default();
        env.insert(
            "z",
            Type::TBox(Box::new(Type::Ref(Lval::new("w", 0), Mutable::Yes))),
            Lifetime(2),
        );
        env.insert(
            "x",
            Type::TBox(Box::new(Type::Ref(Lval::new("y", 3), Mutable::Yes))),
            Lifetime(30),
        );
        assert!(env.read_prohibited(&Lval::new("y", 5)));
    }

    #[test]
    fn basic_write_prohibited() {
        let mut env = Env::default();
        env.insert(
            "z",
            Type::TBox(Box::new(Type::Ref(Lval::new("w", 0), Mutable::Yes))),
            Lifetime(2),
        );
        env.insert(
            "x",
            Type::TBox(Box::new(Type::Ref(Lval::new("y", 3), Mutable::Yes))),
            Lifetime(30),
        );
        assert!(env.write_prohibited(&Lval::new("y", 5)));
    }

    #[test]
    fn basic_write_prohibited_2() {
        let mut env = Env::default();
        env.insert(
            "z",
            Type::TBox(Box::new(Type::Ref(Lval::new("w", 0), Mutable::Yes))),
            Lifetime(2),
        );
        env.insert(
            "x",
            Type::TBox(Box::new(Type::Ref(Lval::new("y", 3), Mutable::No))),
            Lifetime(30),
        );
        assert!(env.write_prohibited(&Lval::new("y", 5)));
    }

    #[test]
    fn move_under_box() {
        let mut env = Env::default();
        env.insert(
            "x",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(Type::Int)))))),
            Lifetime(40),
        );
        assert!(env.moove(&Lval::new("x", 2)).is_ok());
        if let Some(slot) = env.0.get("x") {
            // Expect TBox(TBox(Undefined(TBox(Int))))
            let expected = Type::TBox(Box::new(
                Type::TBox(Box::new(
                    Type::Undefined(Box::new(
                        Type::TBox(Box::new(Type::Int))
                    ))
                ))
            ));
            assert_eq!(slot.tipe, expected);
        } else {
            panic!("x must still be in env");
        }
    }

    #[test]
    fn move_under_ref() {
        let mut env = Env::default();
        env.insert(
            "x",
            Type::TBox(Box::new(Type::Ref(Lval::new("y", 1), Mutable::Yes))),
            Lifetime(40),
        );
        assert!(env.moove(&Lval::new("x", 2)).is_err());
    }

    #[test]
    fn mut_succ() {
        let mut env = Env::default();
        env.insert(
            "x",
            Type::TBox(Box::new(Type::Ref(Lval::new("y", 3), Mutable::Yes))),
            Lifetime(31),
        );
        env.insert("y", Type::Ref(Lval::new("z", 0), Mutable::Yes), Lifetime(24));
        env.insert(
            "z",
            Type::TBox(Box::new(Type::Ref(Lval::new("w", 2), Mutable::Yes))),
            Lifetime(23),
        );
        env.insert(
            "w",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(Type::Int)))))),
            Lifetime(29),
        );
        assert!(env.muut(&Lval::new("x", 3)));
    }

    #[test]
    fn mut_fail() {
        let mut env = Env::default();
        env.insert(
            "x",
            Type::TBox(Box::new(Type::Ref(Lval::new("y", 3), Mutable::Yes))),
            Lifetime(31),
        );
        env.insert("y", Type::Ref(Lval::new("z", 0), Mutable::Yes), Lifetime(24));
        env.insert(
            "z",
            Type::TBox(Box::new(Type::Ref(Lval::new("w", 2), Mutable::No))),
            Lifetime(23),
        );
        env.insert(
            "w",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(Type::Int)))))),
            Lifetime(29),
        );
        assert!(!env.muut(&Lval::new("x", 3)));
    }

    #[test]
    fn compatible_basic() {
        let env = Env::default();
        let t1 = Type::TBox(Box::new(
            Type::TBox(Box::new(
                Type::Undefined(Box::new(
                    Type::TBox(Box::new(Type::Int))
                ))
            ))
        ));
        let t2 = Type::TBox(Box::new(
            Type::Undefined(Box::new(
                Type::TBox(Box::new(
                    Type::TBox(Box::new(Type::Int))
                ))
            ))
        ));
        assert!(env.compatible(&t1, &t2));
    }

    #[test]
    fn compatible_basic_fail() {
        let env = Env::default();
        let t1 = Type::TBox(Box::new(
            Type::TBox(Box::new(
                Type::Undefined(Box::new(
                    Type::TBox(Box::new(Type::Int))
                ))
            ))
        ));
        let t2 = Type::TBox(Box::new(
            Type::Undefined(Box::new(
                Type::TBox(Box::new(Type::Int))
            ))
        ));
        assert!(!env.compatible(&t1, &t2));
    }

    #[test]
    fn compatible_refs() {
        let mut env = Env::default();
        env.insert(
            "y",
            Type::TBox(Box::new(Type::Undefined(Box::new(
                Type::Ref(Lval::new("a", 0), Mutable::No)
            )))),
            Lifetime(1),
        );
        env.insert(
            "z",
            Type::TBox(Box::new(Type::Ref(Lval::new("b", 1), Mutable::No))),
            Lifetime(1),
        );
        env.insert(
            "b",
            Type::TBox(Box::new(Type::Ref(Lval::new("c", 1), Mutable::No))),
            Lifetime(1),
        );
        env.insert("a", Type::Int, Lifetime(1));
        env.insert(
            "c",
            Type::TBox(Box::new(Type::Undefined(Box::new(Type::Int)))),
            Lifetime(1),
        );
        let t1 = Type::TBox(Box::new(Type::Undefined(Box::new(
            Type::Ref(Lval::new("y", 1), Mutable::Yes)
        ))));
        let t2 = Type::TBox(Box::new(Type::Ref(Lval::new("z", 2), Mutable::Yes)));
        assert!(env.compatible(&t1, &t2));
    }

    #[test]
    fn write_basic() {
        let mut env = Env::default();
        env.insert(
            "x",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::Undefined(Box::new(
                Type::TBox(Box::new(Type::Int))
            )))))),
            Lifetime(23),
        );
        assert!(env.write(&Lval::new("x", 2), Type::TBox(Box::new(Type::Int))).is_ok());
        if let Some(slot) = env.0.get("x") {
            assert_eq!(
                slot.tipe,
                Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(Type::Int))))))
            );
        } else {
            panic!("x should still be present");
        }
    }

    #[test]
    fn write_ref() {
        let mut env = Env::default();
        env.insert(
            "x",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::Ref(Lval::new("y", 2), Mutable::Yes))))),
            Lifetime(23),
        );
        env.insert(
            "y",
            Type::TBox(Box::new(Type::Ref(Lval::new("z", 1), Mutable::Yes))),
            Lifetime(11),
        );
        env.insert("z", Type::Ref(Lval::new("w", 2), Mutable::Yes), Lifetime(1));
        env.insert(
            "w",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(
                Type::Ref(Lval::new("a", 0), Mutable::No)
            )))))),
            Lifetime(87),
        );
        env.insert("a", Type::Int, Lifetime(23));
        env.insert("b", Type::Int, Lifetime(44));

        dbg!(&env.0["w"].tipe);
        assert!(env.write(&Lval::new("x", 3), Type::TBox(Box::new(
            Type::Ref(Lval::new("b", 0), Mutable::No)
        ))).is_ok());
        let mut env2 = env.clone();
        // only w’s inner ref should have changed its target from “a” to “b”
        env2.insert(
            "w",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(
                Type::Ref(Lval::new("b", 0), Mutable::No)
            )))))),
            Lifetime(87),
        );
        assert_eq!(env, env2);
    }

    #[test]
    fn drop_basic() {
        let mut env = Env::default();
        env.insert(
            "x",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::Ref(Lval::new("y", 2), Mutable::Yes))))),
            Lifetime(11),
        );
        env.insert(
            "y",
            Type::TBox(Box::new(Type::Ref(Lval::new("z", 1), Mutable::Yes))),
            Lifetime(11),
        );
        env.insert("z", Type::Ref(Lval::new("w", 2), Mutable::Yes), Lifetime(1));
        env.insert(
            "w",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(
                Type::Ref(Lval::new("a", 0), Mutable::No)
            )))))),
            Lifetime(87),
        );
        env.insert("a", Type::Int, Lifetime(11));
        env.insert("b", Type::Int, Lifetime(44));
        env.drop(Lifetime(11));

        let mut env2 = Env::default();
        env2.insert("z", Type::Ref(Lval::new("w", 2), Mutable::Yes), Lifetime(1));
        env2.insert(
            "w",
            Type::TBox(Box::new(Type::TBox(Box::new(Type::TBox(Box::new(
                Type::Ref(Lval::new("a", 0), Mutable::No)
            )))))),
            Lifetime(87),
        );
        env2.insert("b", Type::Int, Lifetime(44));
        assert_eq!(env, env2);
    }
}
