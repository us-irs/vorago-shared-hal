use super::{HwCsProvider, PinMiso, PinMosi, PinSck};
use crate::FunSel;
use crate::gpio::{PinId, PinIdProvider};

use crate::pins::{
    Pa10, Pa11, Pa12, Pa13, Pa14, Pa15, Pa16, Pa17, Pa18, Pa19, Pa20, Pa21, Pa22, Pa23, Pa24, Pa25,
    Pa26, Pa27, Pa28, Pa29, Pa30, Pa31, Pb0, Pb1, Pb2, Pb3, Pb4, Pb5, Pb6, Pb7, Pb8, Pb9, Pb10,
    Pb11, Pb12, Pb13, Pb14, Pb15, Pb16, Pb17, Pb18, Pb19, Pb22, Pb23, Pin,
};

use super::{Bank, HwChipSelectId};

// SPIA

impl PinSck for Pin<Pa31> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunSel = FunSel::Sel1;
}
impl PinMosi for Pin<Pa30> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunSel = FunSel::Sel1;
}
impl PinMiso for Pin<Pa29> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunSel = FunSel::Sel1;
}

impl PinSck for Pin<Pb9> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunSel = FunSel::Sel2;
}
impl PinMosi for Pin<Pb8> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunSel = FunSel::Sel2;
}
impl PinMiso for Pin<Pb7> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunSel = FunSel::Sel2;
}

hw_cs_pins!(
    Bank::Spi0,
    (Pb0, FunSel::Sel2, HwChipSelectId::Id1),
    (Pb1, FunSel::Sel2, HwChipSelectId::Id2),
    (Pb2, FunSel::Sel2, HwChipSelectId::Id3),
    (Pb3, FunSel::Sel2, HwChipSelectId::Id4),
    (Pb4, FunSel::Sel2, HwChipSelectId::Id5),
    (Pb5, FunSel::Sel2, HwChipSelectId::Id6),
    (Pb6, FunSel::Sel2, HwChipSelectId::Id0),
    (Pa24, FunSel::Sel1, HwChipSelectId::Id4),
    (Pa25, FunSel::Sel1, HwChipSelectId::Id3),
    (Pa26, FunSel::Sel1, HwChipSelectId::Id2),
    (Pa27, FunSel::Sel1, HwChipSelectId::Id1),
    (Pa28, FunSel::Sel1, HwChipSelectId::Id0),
);

hw_cs_multi_pin!(
    PinPb0SpiaHwCsId1,
    Pb0,
    Bank::Spi0,
    FunSel::Sel2,
    HwChipSelectId::Id1
);
hw_cs_multi_pin!(
    PinPb1SpiaHwCsId2,
    Pb1,
    Bank::Spi0,
    FunSel::Sel2,
    HwChipSelectId::Id2
);
hw_cs_multi_pin!(
    PinPb2SpiaHwCsId3,
    Pb2,
    Bank::Spi0,
    FunSel::Sel2,
    HwChipSelectId::Id3
);

hw_cs_multi_pin!(
    PinPa21SpiaHwCsId7,
    Pa21,
    Bank::Spi0,
    FunSel::Sel1,
    HwChipSelectId::Id7
);
hw_cs_multi_pin!(
    PinPa22SpiaHwCsId6,
    Pa22,
    Bank::Spi0,
    FunSel::Sel1,
    HwChipSelectId::Id6
);
hw_cs_multi_pin!(
    PinPa23SpiaHwCsId5,
    Pa23,
    Bank::Spi0,
    FunSel::Sel1,
    HwChipSelectId::Id5
);

// SPIB

impl PinSck for Pin<Pa20> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel2;
}
impl PinMosi for Pin<Pa19> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel2;
}
impl PinMiso for Pin<Pa18> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel2;
}

pub type SpiBPortASck = Pin<Pa20>;
pub type SpiBPortAMosi = Pin<Pa19>;
pub type SpiBPortAMiso = Pin<Pa18>;

impl PinSck for Pin<Pb19> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel1;
}
impl PinMosi for Pin<Pb18> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel1;
}
impl PinMiso for Pin<Pb17> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel1;
}

impl PinSck for Pin<Pb5> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel1;
}
impl PinMosi for Pin<Pb4> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel1;
}
impl PinMiso for Pin<Pb3> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunSel = FunSel::Sel1;
}

// TODO: Need to deal with these duplications..
hw_cs_pins!(
    Bank::Spi1,
    (Pb16, FunSel::Sel1, HwChipSelectId::Id0),
    (Pb15, FunSel::Sel1, HwChipSelectId::Id1),
    (Pb14, FunSel::Sel1, HwChipSelectId::Id2),
    (Pb13, FunSel::Sel1, HwChipSelectId::Id3),
    (Pa17, FunSel::Sel2, HwChipSelectId::Id0),
    (Pa16, FunSel::Sel2, HwChipSelectId::Id1),
    (Pa15, FunSel::Sel2, HwChipSelectId::Id2),
    (Pa14, FunSel::Sel2, HwChipSelectId::Id3),
    (Pa13, FunSel::Sel2, HwChipSelectId::Id4),
    (Pa12, FunSel::Sel2, HwChipSelectId::Id5),
    (Pa11, FunSel::Sel2, HwChipSelectId::Id6),
    (Pa10, FunSel::Sel2, HwChipSelectId::Id7),
    (Pa23, FunSel::Sel2, HwChipSelectId::Id5),
    (Pa22, FunSel::Sel2, HwChipSelectId::Id6),
    (Pa21, FunSel::Sel2, HwChipSelectId::Id7),
);

hw_cs_multi_pin!(
    PinPb0SpibHwCsId2,
    Pb0,
    Bank::Spi1,
    FunSel::Sel1,
    HwChipSelectId::Id2
);
hw_cs_multi_pin!(
    PinPb1SpibHwCsId1,
    Pb1,
    Bank::Spi1,
    FunSel::Sel1,
    HwChipSelectId::Id1
);
hw_cs_multi_pin!(
    PinPb2SpibHwCsId0,
    Pb2,
    Bank::Spi1,
    FunSel::Sel1,
    HwChipSelectId::Id0
);

hw_cs_multi_pin!(
    PinPb10SpibHwCsId6,
    Pb10,
    Bank::Spi1,
    FunSel::Sel1,
    HwChipSelectId::Id6
);
hw_cs_multi_pin!(
    PinPb11SpibHwCsId5,
    Pb11,
    Bank::Spi1,
    FunSel::Sel1,
    HwChipSelectId::Id5
);
hw_cs_multi_pin!(
    PinPb12SpibHwCsId4,
    Pb12,
    Bank::Spi1,
    FunSel::Sel1,
    HwChipSelectId::Id4
);

hw_cs_multi_pin!(
    PinPb10SpibHwCsId2,
    Pb10,
    Bank::Spi1,
    FunSel::Sel2,
    HwChipSelectId::Id2
);
hw_cs_multi_pin!(
    PinPb11SpibHwCsId1,
    Pb11,
    Bank::Spi1,
    FunSel::Sel2,
    HwChipSelectId::Id1
);
hw_cs_multi_pin!(
    PinPb12SpibHwCsId0,
    Pb12,
    Bank::Spi1,
    FunSel::Sel2,
    HwChipSelectId::Id0
);

hw_cs_multi_pin!(
    PinPa21SpibHwCsId7,
    Pa21,
    Bank::Spi1,
    FunSel::Sel2,
    HwChipSelectId::Id7
);
hw_cs_multi_pin!(
    PinPa22SpibHwCsId6,
    Pa22,
    Bank::Spi1,
    FunSel::Sel2,
    HwChipSelectId::Id6
);
hw_cs_multi_pin!(
    PinPa23SpibHwCsId5,
    Pa23,
    Bank::Spi1,
    FunSel::Sel2,
    HwChipSelectId::Id5
);

// SPIC

hw_cs_pins!(
    Bank::Spi2,
    (Pb9, FunSel::Sel3, HwChipSelectId::Id1),
    (Pb8, FunSel::Sel3, HwChipSelectId::Id2),
    (Pb7, FunSel::Sel3, HwChipSelectId::Id3),
    (Pb23, FunSel::Sel3, HwChipSelectId::Id2),
    (Pb22, FunSel::Sel3, HwChipSelectId::Id1),
    (Pa20, FunSel::Sel1, HwChipSelectId::Id1),
    (Pa19, FunSel::Sel1, HwChipSelectId::Id2),
    (Pb18, FunSel::Sel1, HwChipSelectId::Id3),
);

hw_cs_multi_pin!(
    PinPa21SpicHwCsId3,
    Pa21,
    Bank::Spi2,
    FunSel::Sel3,
    HwChipSelectId::Id3
);
hw_cs_multi_pin!(
    PinPa22SpicHwCsId2,
    Pa22,
    Bank::Spi2,
    FunSel::Sel3,
    HwChipSelectId::Id2
);
hw_cs_multi_pin!(
    PinPa23SpicHwCsId1,
    Pa23,
    Bank::Spi2,
    FunSel::Sel3,
    HwChipSelectId::Id1
);

hw_cs_multi_pin!(
    PinPa20SpicHwCsId1,
    Pa20,
    Bank::Spi2,
    FunSel::Sel1,
    HwChipSelectId::Id1
);
hw_cs_multi_pin!(
    PinPa20SpicHwCsId4,
    Pa20,
    Bank::Spi2,
    FunSel::Sel3,
    HwChipSelectId::Id4
);
