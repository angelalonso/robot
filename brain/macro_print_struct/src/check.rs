use std::fmt;

macro_rules! MyDisplay {
    ($struct:ident {$( $field:ident:$type:ty ),*,}) => {
        #[derive(Debug)]
        pub struct $struct { pub $($field: $type),*}

        impl fmt::Display for $struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                $(
                    write!(f, "{}: {}\n",
                        stringify!($field).to_string(),
                        match &self.$field {
                            None => "-".to_string(),
                            Some(x) => format!("{:#?}", x)
                        }
                    )?;
                )*
                Ok(())
            }
        }
    };
}

MyDisplay! {
    Check {
        foo: Option<String>,
        bar: Option<String>,
        the_other_one: Option<String>,
    }
}

impl Check {
    pub fn new() -> Result<Self, String> {

        Ok(Self {
            foo: Some("foo new".to_string()),
            bar: Some("bar new".to_string()),
            the_other_one: Some("the other new one".to_string()),
        })
    }

    pub fn show(&mut self) {
        println!("{:?}", self);
    }
}


