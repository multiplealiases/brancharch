use num_enum::FromPrimitive;
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum OpCode {
    #[num_enum(default)]
    Nop,

    LoadA, LoadB, LoadC, LoadD,

    StoreA, StoreB, StoreC, StoreD,

    SetFlag, ClearFlag,

    AddAA, AddBA, AddCA, AddDA,
           AddBB, AddBC, AddBD,
           AddCB, AddCC, AddCD,
           AddDB, AddDC, AddDD,

    SubAA, SubBA, SubCA, SubDA,
           SubBB, SubBC, SubBD,
           SubCB, SubCC, SubCD,
           SubDB, SubDC, SubDD,

    Jmp, JmpRel, WriteIp,

    Branch,

    LessThanA, GreaterThanA, EqualToA,
    LessThanB, GreaterThanB, EqualToB,
    LessThanC, GreaterThanC, EqualToC,
    LessThanD, GreaterThanD, EqualToD,

    AndAA, AndBA, AndCA, AndDA,
           AndBB, AndBC, AndBD,
           AndCB, AndCC, AndCD,
           AndDB, AndDC, AndDD,

    XorAA, XorBA, XorCA, XorDA,
           XorBB, XorBC, XorBD,
           XorCB, XorCC, XorCD,
           XorDB, XorDC, XorDD,

    OrAA,  OrBA,  OrCA,  OrDA,
           OrBB,  OrBC,  OrBD,
           OrCB,  OrCC,  OrCD,
           OrDB,  OrDC,  OrDD,

    RolA, RolB, RolC, RolD,

    RorA, RorB, RorC, RorD,

    Print,

    AttemptHalt = 0xFF,
}
