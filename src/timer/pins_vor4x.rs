use super::{FunSel, TimId, TimPin};
use crate::pins::{
    Pa0, Pa1, Pa2, Pa3, Pa4, Pa5, Pa6, Pa7, Pa8, Pa10, Pa11, Pa12, Pa13, Pa14, Pa15, Pb0, Pb1, Pb2,
    Pb3, Pb4, Pb12, Pb13, Pb14, Pb15, Pc0, Pc1, Pd10, Pd11, Pd12, Pd13, Pd14, Pd15, Pe0, Pe1, Pe2,
    Pe3, Pe4, Pe5, Pe6, Pe7, Pe8, Pe9, Pe12, Pe13, Pe14, Pe15, Pf0, Pf1, Pf9, Pf11, Pf12, Pf13,
    Pf14, Pf15, Pg0, Pg1, Pg2, Pg3, Pg6, Pin, PinId, PinIdProvider,
};
#[cfg(not(feature = "va41628"))]
use crate::pins::{
    Pb5, Pb6, Pb7, Pb8, Pb9, Pb10, Pb11, Pd0, Pd1, Pd2, Pd3, Pd4, Pd5, Pd6, Pd7, Pd8, Pd9, Pe10,
    Pe11, Pf2, Pf3, Pf4, Pf5, Pf6, Pf7, Pf8, Pf10,
};

pin_and_tim!(Pa0, FunSel::Sel1, 0);
pin_and_tim!(Pa1, FunSel::Sel1, 1);
pin_and_tim!(Pa2, FunSel::Sel1, 2);
pin_and_tim!(Pa3, FunSel::Sel1, 3);
pin_and_tim!(Pa4, FunSel::Sel1, 4);
pin_and_tim!(Pa5, FunSel::Sel1, 5);
pin_and_tim!(Pa6, FunSel::Sel1, 6);
pin_and_tim!(Pa7, FunSel::Sel1, 7);
pin_and_tim!(Pa8, FunSel::Sel3, 8);
pin_and_tim!(Pa10, FunSel::Sel2, 23);
pin_and_tim!(Pa11, FunSel::Sel2, 22);
pin_and_tim!(Pa12, FunSel::Sel2, 21);
pin_and_tim!(Pa13, FunSel::Sel2, 20);
pin_and_tim!(Pa14, FunSel::Sel2, 19);
pin_and_tim!(Pa15, FunSel::Sel2, 18);

pin_and_tim!(Pb0, FunSel::Sel2, 17);
pin_and_tim!(Pb1, FunSel::Sel2, 16);
pin_and_tim!(Pb2, FunSel::Sel2, 15);
pin_and_tim!(Pb3, FunSel::Sel2, 14);
pin_and_tim!(Pb4, FunSel::Sel2, 13);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pb5, FunSel::Sel2, 12);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pb6, FunSel::Sel2, 11);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pb7, FunSel::Sel2, 10);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pb8, FunSel::Sel2, 9);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pb9, FunSel::Sel2, 8);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pb10, FunSel::Sel2, 7);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pb11, FunSel::Sel2, 6);
pin_and_tim!(Pb12, FunSel::Sel2, 5);
pin_and_tim!(Pb13, FunSel::Sel2, 4);
pin_and_tim!(Pb14, FunSel::Sel2, 3);
pin_and_tim!(Pb15, FunSel::Sel2, 2);

pin_and_tim!(Pc0, FunSel::Sel2, 1);
pin_and_tim!(Pc1, FunSel::Sel2, 0);

#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd0, FunSel::Sel2, 0);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd1, FunSel::Sel2, 1);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd2, FunSel::Sel2, 2);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd3, FunSel::Sel2, 3);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd4, FunSel::Sel2, 4);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd5, FunSel::Sel2, 5);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd6, FunSel::Sel2, 6);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd7, FunSel::Sel2, 7);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd8, FunSel::Sel2, 8);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pd9, FunSel::Sel2, 9);
pin_and_tim!(Pd10, FunSel::Sel2, 10);
pin_and_tim!(Pd11, FunSel::Sel2, 11);
pin_and_tim!(Pd12, FunSel::Sel2, 12);
pin_and_tim!(Pd13, FunSel::Sel2, 13);
pin_and_tim!(Pd14, FunSel::Sel2, 14);
pin_and_tim!(Pd15, FunSel::Sel2, 15);

pin_and_tim!(Pe0, FunSel::Sel2, 16);
pin_and_tim!(Pe1, FunSel::Sel2, 17);
pin_and_tim!(Pe2, FunSel::Sel2, 18);
pin_and_tim!(Pe3, FunSel::Sel2, 19);
pin_and_tim!(Pe4, FunSel::Sel2, 20);
pin_and_tim!(Pe5, FunSel::Sel2, 21);
pin_and_tim!(Pe6, FunSel::Sel2, 22);
pin_and_tim!(Pe7, FunSel::Sel2, 23);
pin_and_tim!(Pe8, FunSel::Sel3, 16);
pin_and_tim!(Pe9, FunSel::Sel3, 17);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pe10, FunSel::Sel3, 18);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pe11, FunSel::Sel3, 19);
pin_and_tim!(Pe12, FunSel::Sel3, 20);
pin_and_tim!(Pe13, FunSel::Sel3, 21);
pin_and_tim!(Pe14, FunSel::Sel3, 22);
pin_and_tim!(Pe15, FunSel::Sel3, 23);

pin_and_tim!(Pf0, FunSel::Sel3, 0);
pin_and_tim!(Pf1, FunSel::Sel3, 1);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf2, FunSel::Sel3, 2);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf3, FunSel::Sel3, 3);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf4, FunSel::Sel3, 4);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf5, FunSel::Sel3, 5);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf6, FunSel::Sel3, 6);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf7, FunSel::Sel3, 7);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf8, FunSel::Sel3, 8);
pin_and_tim!(Pf9, FunSel::Sel3, 9);
#[cfg(not(feature = "va41628"))]
pin_and_tim!(Pf10, FunSel::Sel3, 10);
pin_and_tim!(Pf11, FunSel::Sel3, 11);
pin_and_tim!(Pf12, FunSel::Sel3, 12);
pin_and_tim!(Pf13, FunSel::Sel2, 19);
pin_and_tim!(Pf14, FunSel::Sel2, 20);
pin_and_tim!(Pf15, FunSel::Sel2, 21);

pin_and_tim!(Pg0, FunSel::Sel2, 22);
pin_and_tim!(Pg1, FunSel::Sel2, 23);
pin_and_tim!(Pg2, FunSel::Sel1, 9);
pin_and_tim!(Pg3, FunSel::Sel1, 10);
pin_and_tim!(Pg6, FunSel::Sel1, 12);
