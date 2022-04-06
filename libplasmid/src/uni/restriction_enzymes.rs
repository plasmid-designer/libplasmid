#![allow(non_upper_case_globals)]

use crate::uni::{IupacNucleotide, IupacNucleotide::*};

#[derive(Debug, PartialEq, Eq)]
pub enum CutMode {
    /// Cut in the middle
    M,
    /// Cut before and after
    A,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RestrictionEnzyme {
    name: String,
    before: Vec<IupacNucleotide>,
    after: Vec<IupacNucleotide>,
    mode: CutMode,
}

impl RestrictionEnzyme {
    pub fn new(name: &str, before: &[IupacNucleotide], after: &[IupacNucleotide], mode: CutMode) -> Self {
        Self {
            name: name.to_string(),
            before: Vec::from_iter(before.iter().map(|&n| n)),
            after: Vec::from_iter(after.iter().map(|&n| n)),
            mode,
        }
    }
}

macro_rules! define_enzyme {
    ($mode:ident; $name:ident: $before:expr, $after:expr) => {
        RestrictionEnzyme::new(stringify!($name), &$before, &$after, CutMode::$mode)
    }
}

lazy_static! {
    pub static ref RestrictionEnzymes: Vec<RestrictionEnzyme> = vec![
        define_enzyme!(M; AclI: [A,A], [C,G,T,T]),
        define_enzyme!(M; HindIII: [A], [A,G,C,T,T]),
        define_enzyme!(M; SspI: [A,A,T], [A,T,T]),
        define_enzyme!(M; MluCI: [], [A,A,T,T]),
        define_enzyme!(M; PciI: [A], [C,A,T,G,T]),
        define_enzyme!(M; AgeI: [A], [C,C,G,G,T]),
        define_enzyme!(M; SexAI: [A], [C,C,W,G,G,T]),
        define_enzyme!(M; MluI: [A], [C,G,C,G,T]),
        define_enzyme!(M; HpyCH4IV: [A], [C,G,T]),
        define_enzyme!(M; HpyCH4III: [A,C,N], [G,T]),
        define_enzyme!(M; AflIII: [A], [C,R,Y,G,T]),
        define_enzyme!(M; SpeI: [A], [C,T,A,G,T]),
        define_enzyme!(M; BglII: [A], [G,A,T,C,T]),
        define_enzyme!(M; AfeI: [A,G,C], [G,C,T]),
        define_enzyme!(M; AluI: [A,G], [C,T]),
        define_enzyme!(M; StuI: [A,G,G], [C,C,T]),
        define_enzyme!(M; ScaI: [A,G,T], [A,C,T]),
        define_enzyme!(M; ClaI: [A,T], [C,G,A,T]),
        define_enzyme!(M; BspDI: [A,T], [C,G,A,T]),
        define_enzyme!(M; NsiI: [A,T,G,C,A], [T]),
        define_enzyme!(M; AseI: [A,T], [T,A,A,T]),
        define_enzyme!(M; SwaI: [A,T,T,T], [A,A,A,T]),
        define_enzyme!(M; MfeI: [C], [A,A,T,T,G]),
        define_enzyme!(M; NbBssSI: [C,A,C,G,A,G], []),
        define_enzyme!(M; PmlI: [C,A,C], [G,T,G]),
        define_enzyme!(M; DraIII: [C,A,C,N,N,N], [G,T,G]),
        define_enzyme!(M; AleI_v2: [C,A,C,N,N], [N,N,G,T,G]),
        define_enzyme!(M; PvuII: [C,A,G], [C,T,G]),
        define_enzyme!(M; AlwNI: [C,A,G,N,N,N], [C,T,G]),
        define_enzyme!(M; NdeI: [C,A], [T,A,T,G]),
        define_enzyme!(M; FatI: [], [C,A,T,G]),
        define_enzyme!(M; CviAII: [C], [A,T,G]),
        define_enzyme!(M; NlaIII: [C,A,T,G], []),
        define_enzyme!(M; MslI: [C,A,Y,N,N], [N,N,R,T,G]),
        define_enzyme!(M; XcmI: [C,C,A,N,N,N,N,N], [N,N,N,N,T,G,G]),
        define_enzyme!(M; BstXI: [C,C,A,N,N,N,N,N], [N,T,G,G]),
        define_enzyme!(M; PflMI: [C,C,A,N,N,N,N], [N,T,G,G]),
        define_enzyme!(M; NcoI: [C], [C,A,T,G,G]),
        define_enzyme!(M; SmaI: [C,C,C], [G,G,G]),
        define_enzyme!(M; TspMI: [C], [C,C,G,G,G]),
        define_enzyme!(M; XmaI: [C], [C,C,G,G,G]),
        define_enzyme!(M; SacII: [C,C,G,C], [G,G]),
        define_enzyme!(M; MspI: [C], [C,G,G]),
        define_enzyme!(M; HpaII: [C], [C,G,G]),
        define_enzyme!(M; StyD4I: [], [C,C,N,G,G]),
        define_enzyme!(M; ScrFI: [C,C], [N,G,G]),
        define_enzyme!(M; BsaJI: [C], [C,N,N,G,G]),
        define_enzyme!(M; BslI: [C,C,N,N,N,N,N], [N,N,G,G]),
        define_enzyme!(M; BtgI: [C], [C,R,Y,G,G]),
        define_enzyme!(M; NciI: [C,C], [S,G,G]),
        define_enzyme!(M; AvrII: [C], [C,T,A,G,G]),
        define_enzyme!(M; NbBbvCI: [C,C,T,C,A,G,C], []),
        define_enzyme!(M; SbfI: [C,C,T,G,C,A], [G,G]),
        define_enzyme!(M; Bsu36I: [C,C], [T,N,A,G,G]),
        define_enzyme!(M; EcoNI: [C,C,T,N,N], [N,N,N,A,G,G]),
        define_enzyme!(M; PspGI: [], [C,C,W,G,G]),
        define_enzyme!(M; BstNI: [C,C], [W,G,G]),
        define_enzyme!(M; StyI: [C], [C,W,W,G,G]),
        define_enzyme!(M; PvuI: [C,G,A,T], [C,G]),
        define_enzyme!(M; BstUI: [C,G], [C,G]),
        define_enzyme!(M; EagI: [C], [G,G,C,C,G]),
        define_enzyme!(M; RsrII: [C,G], [G,W,C,C,G]),
        define_enzyme!(M; BsiEI: [C,G,R,Y], [C,G]),
        define_enzyme!(M; BsiWI: [C], [G,T,A,C,G]),
        define_enzyme!(M; BsmBI_v2: [C,G,T,C,T,C], []),
        define_enzyme!(M; Hpy99I: [C,G,W,C,G], []),
        define_enzyme!(M; MspA1I: [C,M,G], [C,K,G]),
        define_enzyme!(M; AbaSI: [C,N,N,N,N,N,N,N,N,N,N,N], [N,N,N,N,N,N,N,N,N,G]),
        define_enzyme!(M; SgrAI: [C,R], [C,C,G,G,Y,G]),
        define_enzyme!(M; BfaI: [C], [T,A,G]),
        define_enzyme!(M; XhoI: [C], [T,C,G,A,G]),
        define_enzyme!(M; PaeR7I: [C], [T,C,G,A,G]),
        define_enzyme!(M; PstI: [C,T,G,C,A], [G]),
        define_enzyme!(M; DdeI: [C], [T,N,A,G]),
        define_enzyme!(M; SfcI: [C], [T,R,Y,A,G]),
        define_enzyme!(M; AflII: [C], [T,T,A,A,G]),
        define_enzyme!(M; SmlI: [C], [T,Y,R,A,G]),
        define_enzyme!(M; BsoBI: [C], [Y,C,G,R,G]),
        define_enzyme!(M; AvaI: [C], [Y,C,G,R,G]),
        define_enzyme!(M; XmnI: [G,A,A,N,N], [N,N,T,T,C]),
        define_enzyme!(M; NbBsmI: [G,A,A,T,G,C], []),
        define_enzyme!(M; EcoRI: [G], [A,A,T,T,C]),
        define_enzyme!(M; AatII: [G,A,C,G,T], [C]),
        define_enzyme!(M; ZraI: [G,A,C], [G,T,C]),
        define_enzyme!(M; PflFI: [G,A,C,N], [N,N,G,T,C]),
        define_enzyme!(M; Tth111I: [G,A,C,N], [N,N,G,T,C]),
        define_enzyme!(M; PshAI: [G,A,C,N,N], [N,N,G,T,C]),
        define_enzyme!(M; AhdI: [G,A,C,N,N,N], [N,N,G,T,C]),
        define_enzyme!(M; DrdI: [G,A,C,N,N,N,N], [N,N,G,T,C]),
        define_enzyme!(M; Eco53kI: [G,A,G], [C,T,C]),
        define_enzyme!(M; SacI: [G,A,G,C,T], [C]),
        define_enzyme!(M; HinfI: [G], [A,N,T,C]),
        define_enzyme!(M; EcoRV: [G,A,T], [A,T,C]),
        define_enzyme!(M; DpnII: [], [G,A,T,C]),
        define_enzyme!(M; MboI: [], [G,A,T,C]),
        define_enzyme!(M; Sau3AI: [], [G,A,T,C]),
        define_enzyme!(M; DpnI: [G,A], [T,C]),
        define_enzyme!(M; BsaBI: [G,A,T,N,N], [N,N,A,T,C]),
        define_enzyme!(M; TfiI: [G], [A,W,T,C]),
        define_enzyme!(M; NbBsrDI: [G,C,A,A,T,G], []),
        define_enzyme!(M; NbBtsI: [G,C,A,G,T,G], []),
        define_enzyme!(M; BstAPI: [G,C,A,N,N,N,N], [N,T,G,C]),
        define_enzyme!(M; SphI: [G,C,A,T,G], [C]),
        define_enzyme!(M; SrfI: [G,C,C,C], [G,G,G,C]),
        define_enzyme!(M; NgoMIV: [G], [C,C,G,G,C]),
        define_enzyme!(M; NaeI: [G,C,C], [G,G,C]),
        define_enzyme!(M; BglI: [G,C,C,N,N,N,N], [N,G,G,C]),
        define_enzyme!(M; AsiSI: [G,C,G,A,T], [C,G,C]),
        define_enzyme!(M; HhaI: [G,C,G], [C]),
        define_enzyme!(M; HinP1I: [G], [C,G,C]),
        define_enzyme!(M; BssHII: [G], [C,G,C,G,C]),
        define_enzyme!(M; NotI: [G,C], [G,G,C,C,G,C]),
        define_enzyme!(M; Fnu4HI: [G,C], [N,G,C]),
        define_enzyme!(M; Cac8I: [G,C,N], [N,G,C]),
        define_enzyme!(M; MwoI: [G,C,N,N,N,N,N], [N,N,G,C]),
        define_enzyme!(M; BmtI: [G,C,T,A,G], [C]),
        define_enzyme!(M; NheI: [G], [C,T,A,G,C]),
        define_enzyme!(M; BlpI: [G,C], [T,N,A,G,C]),
        define_enzyme!(M; TseI: [G], [C,W,G,C]),
        define_enzyme!(M; ApeKI: [G], [C,W,G,C]),
        define_enzyme!(M; Bsp1286I: [G,D,G,C,H], [C]),
        define_enzyme!(M; BamHI: [G], [G,A,T,C,C]),
        define_enzyme!(M; HaeIII: [G,G], [C,C]),
        define_enzyme!(M; FseI: [G,G,C,C,G,G], [C,C]),
        define_enzyme!(M; SfiI: [G,G,C,C,N,N,N,N], [N,G,G,C,C]),
        define_enzyme!(M; NarI: [G,G], [C,G,C,C]),
        define_enzyme!(M; SfoI: [G,G,C], [G,C,C]),
        define_enzyme!(M; KasI: [G], [G,C,G,C,C]),
        define_enzyme!(M; PluTI: [G,G,C,G,C], [C]),
        define_enzyme!(M; AscI: [G,G], [C,G,C,G,C,C]),
        define_enzyme!(M; PspOMI: [G], [G,G,C,C,C]),
        define_enzyme!(M; ApaI: [G,G,G,C,C], [C]),
        define_enzyme!(M; Sau96I: [G], [G,N,C,C]),
        define_enzyme!(M; NlaIV: [G,G,N], [N,C,C]),
        define_enzyme!(M; Acc65I: [G], [G,T,A,C,C]),
        define_enzyme!(M; KpnI: [G,G,T,A,C], [C]),
        define_enzyme!(M; BstEII: [G], [G,T,N,A,C,C]),
        define_enzyme!(M; AvaII: [G], [G,W,C,C]),
        define_enzyme!(M; BanI: [G], [G,Y,R,C,C]),
        define_enzyme!(M; BaeGI: [G,K,G,C,M], [C]),
        define_enzyme!(M; BsaHI: [G,R], [C,G,Y,C]),
        define_enzyme!(M; BanII: [G,R,G,C,Y], [C]),
        define_enzyme!(M; CviQI: [G], [T,A,C]),
        define_enzyme!(M; RsaI: [G,T], [A,C]),
        define_enzyme!(M; BstZ17I: [G,T,A,T,A,C], []),
        define_enzyme!(M; SalI: [G], [T,C,G,A,C]),
        define_enzyme!(M; ApaLI: [G], [T,G,C,A,C]),
        define_enzyme!(M; AccI: [G,T], [M,K,A,C]),
        define_enzyme!(M; Hpy166II: [G,T,N], [N,A,C]),
        define_enzyme!(M; Tsp45I: [], [G,T,S,A,C]),
        define_enzyme!(M; HpaI: [G,T,T], [A,A,C]),
        define_enzyme!(M; PmeI: [G,T,T,T], [A,A,A,C]),
        define_enzyme!(M; HincII: [G,T,Y], [R,A,C]),
        define_enzyme!(M; BsiHKAI: [G,W,G,C,W], [C]),
        define_enzyme!(M; TspRI: [N,N,C,A,S,T,G,N,N], []),
        define_enzyme!(M; ApoI_HF: [R], [A,A,T,T,Y]),
        define_enzyme!(M; ApoI: [R], [A,A,T,T,Y]),
        define_enzyme!(M; NspI: [R,C,A,T,G], [Y]),
        define_enzyme!(M; BsrFI_v2: [R], [C,C,G,G,Y]),
        define_enzyme!(M; BstYI: [R], [G,A,T,C,Y]),
        define_enzyme!(M; HaeII: [R,G,C,G,C], [Y]),
        define_enzyme!(M; CviKI_1: [R,G], [C,Y]),
        define_enzyme!(M; EcoO109I: [R,G], [G,N,C,C,Y]),
        define_enzyme!(M; PpuMI: [R,G], [G,W,C,C,Y]),
        define_enzyme!(M; SnaBI: [T,A,C], [G,T,A]),
        define_enzyme!(M; BspHI: [T], [C,A,T,G,A]),
        define_enzyme!(M; BspEI: [T], [C,C,G,G,A]),
        define_enzyme!(M; TaqI_v2: [T], [C,G,A]),
        define_enzyme!(M; NruI: [T,C,G], [C,G,A]),
        define_enzyme!(M; Hpy188I: [T,C,N], [G,A]),
        define_enzyme!(M; Hpy188III: [T,C], [N,N,G,A]),
        define_enzyme!(M; XbaI: [T], [C,T,A,G,A]),
        define_enzyme!(M; BclI: [T], [G,A,T,C,A]),
        define_enzyme!(M; BclI_HF: [T], [G,A,T,C,A]),
        define_enzyme!(M; HpyCH4V: [T,G], [C,A]),
        define_enzyme!(M; FspI: [T,G,C], [G,C,A]),
        define_enzyme!(M; MscI: [T,G,G], [C,C,A]),
        define_enzyme!(M; BsrGI: [T], [G,T,A,C,A]),
        define_enzyme!(M; MseI: [T], [T,A,A]),
        define_enzyme!(M; PacI: [T,T,A,A,T], [T,A,A]),
        define_enzyme!(M; PsiI_v2: [T,T,A], [T,A,A]),
        define_enzyme!(M; BstBI: [T,T], [C,G,A,A]),
        define_enzyme!(M; DraI: [T,T,T], [A,A,A]),
        define_enzyme!(M; PspXI: [V,C], [T,C,G,A,G,B]),
        define_enzyme!(M; BsaWI: [W], [C,C,G,G,W]),
        define_enzyme!(M; BsaAI: [Y,A,C], [G,T,R]),
        define_enzyme!(M; EaeI: [Y], [G,G,C,C,R]),
    ];
}