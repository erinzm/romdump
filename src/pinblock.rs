macro_rules! pin_block_struct {
    ($name:ident, $pintrait:ident, [$($pin:ident: $pinp:ident),*]) => {
        pub struct $name<$($pinp,)*> {
            $(
                $pin: $pinp,
            )*
        }

        impl<$($pinp,)*> $name<$($pinp,)*>
        where
            $($pinp: $pintrait,)*
        {
            pub fn new($($pin: $pinp, )*) -> Self {
                Self { $($pin,)* }
            }
        }
    }
}

#[macro_export]
macro_rules! pin_block {
    {$name:ident: [impl $pintrait:ident] = [$($pin:ident: $pinp:ident),*]; write($writety:ty) { $($writepin:ident = $writebitpos:expr;)* }} => {
        pin_block_struct!($name, $pintrait, [$($pin: $pinp),*]);

        impl<$($pinp,)*> $name<$($pinp,)*>
        where
            $($pinp: $pintrait,)*
        {
            pub fn write(&mut self, v: $writety) {
                $(
                    self.$writepin.set_value(v & (1 << $writebitpos) != 0);
                )*
            }
        }
    }
}