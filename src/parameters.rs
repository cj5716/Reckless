#[allow(unused_macros)]
#[cfg(not(feature = "spsa"))]
macro_rules! define {
    {$($type:ident $name:ident: $value:expr; )*} => {
        $(pub const fn $name() -> $type {
            $value
        })*
    };
}

#[cfg(feature = "spsa")]
macro_rules! define {
    {$($type:ident $name:ident: $value:expr; )*} => {
        pub fn set_parameter(name: &str, value: &str) {
            match name {
                $(stringify!($name) => unsafe { parameters::$name = value.parse().unwrap() },)*
                _ => panic!("Unknown tunable parameter: {name}"),
            }
        }

        pub fn print_options() {
            $(println!("option name {} type string", stringify!($name));)*
        }

        $(pub fn $name() -> $type {
            unsafe { parameters::$name }
        })*

        #[allow(non_upper_case_globals)]
        mod parameters {
            $(pub static mut $name: $type = $value;)*
        }
    };
}

define!(
	i32 lmr1: 4096;
	i32 lmr2: 519;
	i32 lmr3: 437;
	i32 lmr4: 333;
	i32 lmr5: 955;
	i32 lmr6: 2048;
	i32 lmr7: 425;
	i32 lmr8: -241;
	i32 lmr9: 1155;
	i32 lmr10: 3417;
	i32 lmr11: 1412;
	i32 lmr12: 464;
	i32 lmr13: 326;
	i32 lmr14: 1024;
	i32 lmr15: 2171;
	i32 lmr16: 179;
	i32 lmr17: 418;
	i32 lmr18: -65;
	i32 lmr19: 91;
	i32 lmr20: 1426;
	i32 lmr21: 130;
	i32 lmr22: 611;
	i32 lmr23: 685;
	i32 lmr24: 1852;
	i32 lmr25: 2204;
	i32 lmr26: 1151;
	i32 lmr27: 400;
	i32 lmr28: 496;
	i32 lmr29: 185;
	i32 lmr30: 2021;
	i32 lmr31: 256;
	i32 lmr32: 256;
	i32 lmr33: 4096;
);