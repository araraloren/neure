use neure::prelude::*;

#[test]
fn into() {
    assert!(into_impl().is_ok());
}

fn into_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Ty<'a> {
        Layer0(&'a str),

        Layer1(&'a str, &'a str),

        Layer2(&'a str, &'a str, &'a str),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Field<'a> {
        ident: &'a str,

        ty_name: Ty<'a>,

        public: bool,
    }

    impl<'a> Field<'a> {
        pub fn private(ident: &'a str, ty_name: Ty<'a>) -> Self {
            Self {
                ident,
                ty_name,
                public: false,
            }
        }

        pub fn public(name: &'a str, ty_name: Ty<'a>) -> Self {
            Self {
                ident: name,
                ty_name,
                public: true,
            }
        }
    }

    let unit = neu::ascii_alphabetic()
        .or(neu::ascii_alphanumeric())
        .or('_');
    let ident = unit.repeat_one_more();
    let ty = neu::ascii_alphabetic()
        .or('_')
        .repeat_one()
        .then(ident)
        .pat();
    let layer0 = ty.try_map(|ty| Ok(Ty::Layer0(ty)));
    let layer1 = ctor::Wrap::dyn_rc(ty.then(ty.quote("<", ">")))
        // Add into_dyn_* reduce the trait solve time
        .try_map(|(w, ty)| Ok(Ty::Layer1(w, ty)));
    let layer2 = ctor::Wrap::dyn_rc(ty.then(ty.then(ty.quote("<", ">")).quote("<", ">"))) // Add into_dyn_* reduce the trait solve time
        .try_map(|(w1, (w2, ty))| Ok(Ty::Layer2(w1, w2, ty)));
    let field = ident.sep_once(":", layer2.or(layer1.or(layer0)));
    let public = field
        .clone()
        .padded("pub")
        .try_map(|(name, ty_name)| Ok(Field::public(name, ty_name)));
    let private = field.try_map(|(name, ty_name)| Ok(Field::private(name, ty_name)));
    let parser = public.or(private).sep(",");
    let space = neu::whitespace().repeat_full();
    let fields = CharsCtx::new("a: i64, b: Option<String>, pub c: bool")
        .skip_before(space)
        .ctor(&parser)?;

    assert_eq!(
        fields,
        vec![
            Field {
                ident: "a",
                ty_name: Ty::Layer0("i64"),
                public: false
            },
            Field {
                ident: "b",
                ty_name: Ty::Layer1("Option", "String"),
                public: false
            },
            Field {
                ident: "c",
                ty_name: Ty::Layer0("bool"),
                public: true
            }
        ]
    );
    Ok(())
}
