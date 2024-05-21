impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        match byte {
            0  => OpCode::Nop,

            1  => OpCode::LoadA,
            2  => OpCode::LoadB,
            3  => OpCode::LoadC,
            4  => OpCode::LoadD,

            5  => OpCode::StoreA,
            6  => OpCode::StoreB,
            7  => OpCode::StoreC,
            8  => OpCode::StoreD,

            9  => OpCode::SetFlag,
            10 => OpCode::ClearFlag,

            11 => OpCode::AddAA,
            12 => OpCode::AddBA,
            13 => OpCode::AddCA,
            14 => OpCode::AddDA,
            15 => OpCode::AddBB,
            16 => OpCode::AddBC,
            17 => OpCode::AddBD,
            18 => OpCode::AddCB,
            19 => OpCode::AddCC,
            20 => OpCode::AddCD,
            21 => OpCode::AddDB,
            22 => OpCode::AddDC,
            23 => OpCode::AddDD,

            24 => OpCode::SubAA,
            25 => OpCode::SubBA,
            26 => OpCode::SubCA,
            27 => OpCode::SubDA,
            28 => OpCode::SubBB,
            29 => OpCode::SubBC,
            30 => OpCode::SubBD,
            31 => OpCode::SubCB,
            32 => OpCode::SubCC,
            33 => OpCode::SubCD,
            34 => OpCode::SubDB,
            35 => OpCode::SubDC,
            36 => OpCode::SubDD,

            37 => OpCode::Jmp,
            38 => OpCode::JmpRel,
            39 => OpCode::WriteIp,

            40 => OpCode::Branch,

            41 => OpCode::LessThanA,
            42 => OpCode::GreaterThanA, 
            43 => OpCode::EqualToA,
            44 => OpCode::LessThanB,
            45 => OpCode::GreaterThanB,
            46 => OpCode::EqualToB,
            47 => OpCode::LessThanC,
            48 => OpCode::GreaterThanC,
            49 => OpCode::EqualToC,
            50 => OpCode::LessThanD,
            51 => OpCode::GreaterThanD,
            52 => OpCode::EqualToD,            

            _ => OpCode::Nop
        }
    }
}

#[repr(u8)]
pub enum OpCode {
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
}
