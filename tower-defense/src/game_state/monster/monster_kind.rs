use crate::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, State)]
pub enum MonsterKind {
    Mob01,
    Mob02,
    Mob03,
    Mob04,
    Mob05,
    Mob06,
    Mob07,
    Mob08,
    Mob09,
    Mob10,
    Mob11,
    Mob12,
    Mob13,
    Mob14,
    Mob15,
    Mob16,
    Mob17,
    Mob18,
    Mob19,
    Mob20,
    Mob21,
    Mob22,
    Mob23,
    Mob24,
    Mob25,
    Mob26,
    Mob27,
    Mob28,
    Mob29,
    Mob30,
    Mob31,
    Mob32,
    Mob33,
    Mob34,
    Mob35,
    Mob36,
    Mob37,
    Mob38,
    Mob39,
    Mob40,
    Mob41,
    Mob42,
    Mob43,
    Mob44,
    Mob45,
    Mob46,
    Mob47,
    Mob48,
    Mob49,
    Mob50,
    Boss01,
    Boss02,
    Boss03,
    Boss04,
    Boss05,
    Boss06,
    Boss07,
    Boss08,
    Boss09,
    Boss10,
    Boss11,
}

impl MonsterKind {
    pub fn image(self) -> Image {
        match self {
            MonsterKind::Mob01 => crate::asset::image::monster::MOB01,
            MonsterKind::Mob02 => crate::asset::image::monster::MOB02,
            MonsterKind::Mob03 => crate::asset::image::monster::MOB03,
            MonsterKind::Mob04 => crate::asset::image::monster::MOB04,
            MonsterKind::Mob05 => crate::asset::image::monster::MOB05,
            MonsterKind::Mob06 => crate::asset::image::monster::MOB06,
            MonsterKind::Mob07 => crate::asset::image::monster::MOB07,
            MonsterKind::Mob08 => crate::asset::image::monster::MOB08,
            MonsterKind::Mob09 => crate::asset::image::monster::MOB09,
            MonsterKind::Mob10 => crate::asset::image::monster::MOB10,
            MonsterKind::Mob11 => crate::asset::image::monster::MOB11,
            MonsterKind::Mob12 => crate::asset::image::monster::MOB12,
            MonsterKind::Mob13 => crate::asset::image::monster::MOB13,
            MonsterKind::Mob14 => crate::asset::image::monster::MOB14,
            MonsterKind::Mob15 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob16 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob17 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob18 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob19 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob20 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob21 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob22 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob23 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob24 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob25 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob26 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob27 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob28 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob29 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob30 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob31 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob32 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob33 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob34 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob35 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob36 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob37 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob38 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob39 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob40 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob41 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob42 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob43 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob44 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob45 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob46 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob47 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob48 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob49 => crate::asset::image::monster::MOB15,
            MonsterKind::Mob50 => crate::asset::image::monster::MOB15,
            MonsterKind::Boss01 => crate::asset::image::monster::BOSS01,
            MonsterKind::Boss02 => crate::asset::image::monster::BOSS02,
            MonsterKind::Boss03 => crate::asset::image::monster::BOSS03,
            MonsterKind::Boss04 => crate::asset::image::monster::BOSS04,
            MonsterKind::Boss05 => crate::asset::image::monster::BOSS05,
            MonsterKind::Boss06 => crate::asset::image::monster::BOSS06,
            MonsterKind::Boss07 => crate::asset::image::monster::BOSS07,
            MonsterKind::Boss08 => crate::asset::image::monster::BOSS08,
            MonsterKind::Boss09 => crate::asset::image::monster::BOSS09,
            MonsterKind::Boss10 => crate::asset::image::monster::BOSS10,
            MonsterKind::Boss11 => crate::asset::image::monster::BOSS11,
        }
    }

    pub fn is_normal_monster(&self) -> bool {
        matches!(
            self,
            MonsterKind::Mob01
                | MonsterKind::Mob02
                | MonsterKind::Mob03
                | MonsterKind::Mob04
                | MonsterKind::Mob05
                | MonsterKind::Mob06
                | MonsterKind::Mob07
                | MonsterKind::Mob08
                | MonsterKind::Mob09
                | MonsterKind::Mob10
                | MonsterKind::Mob11
                | MonsterKind::Mob12
                | MonsterKind::Mob13
                | MonsterKind::Mob14
                | MonsterKind::Mob15
                | MonsterKind::Mob16
                | MonsterKind::Mob17
                | MonsterKind::Mob18
                | MonsterKind::Mob19
                | MonsterKind::Mob20
                | MonsterKind::Mob21
                | MonsterKind::Mob22
                | MonsterKind::Mob23
                | MonsterKind::Mob24
                | MonsterKind::Mob25
                | MonsterKind::Mob26
                | MonsterKind::Mob27
                | MonsterKind::Mob28
                | MonsterKind::Mob29
                | MonsterKind::Mob30
                | MonsterKind::Mob31
                | MonsterKind::Mob32
                | MonsterKind::Mob33
                | MonsterKind::Mob34
                | MonsterKind::Mob35
                | MonsterKind::Mob36
                | MonsterKind::Mob37
                | MonsterKind::Mob38
                | MonsterKind::Mob39
                | MonsterKind::Mob40
                | MonsterKind::Mob41
                | MonsterKind::Mob42
                | MonsterKind::Mob43
                | MonsterKind::Mob44
                | MonsterKind::Mob45
                | MonsterKind::Mob46
                | MonsterKind::Mob47
                | MonsterKind::Mob48
                | MonsterKind::Mob49
                | MonsterKind::Mob50
        )
    }

    pub fn emoji(&self) -> &str {
        match self {
            MonsterKind::Mob01 => "🌱",
            MonsterKind::Mob02 => "🌿",
            MonsterKind::Mob03 => "🌵",
            MonsterKind::Mob04 => "🍀",
            MonsterKind::Mob05 => "💮",
            MonsterKind::Mob06 => "🥬",
            MonsterKind::Mob07 => "🥒",
            MonsterKind::Mob08 => "🫐",
            MonsterKind::Mob09 => "🍇",
            MonsterKind::Mob10 => "🫑",
            MonsterKind::Mob11 => "🍒",
            MonsterKind::Mob12 => "🐝",
            MonsterKind::Mob13 => "🪲",
            MonsterKind::Mob14 => "🪳",
            MonsterKind::Mob15 => "🐨",
            MonsterKind::Mob16 => "🐨",
            MonsterKind::Mob17 => "🐨",
            MonsterKind::Mob18 => "🐨",
            MonsterKind::Mob19 => "🐨",
            MonsterKind::Mob20 => "🐨",
            MonsterKind::Mob21 => "🐨",
            MonsterKind::Mob22 => "🐨",
            MonsterKind::Mob23 => "🐨",
            MonsterKind::Mob24 => "🐨",
            MonsterKind::Mob25 => "🐨",
            MonsterKind::Mob26 => "🐨",
            MonsterKind::Mob27 => "🐨",
            MonsterKind::Mob28 => "🐨",
            MonsterKind::Mob29 => "🐨",
            MonsterKind::Mob30 => "🐨",
            MonsterKind::Mob31 => "🐨",
            MonsterKind::Mob32 => "🐨",
            MonsterKind::Mob33 => "🐨",
            MonsterKind::Mob34 => "🐨",
            MonsterKind::Mob35 => "🐨",
            MonsterKind::Mob36 => "🐨",
            MonsterKind::Mob37 => "🐨",
            MonsterKind::Mob38 => "🐨",
            MonsterKind::Mob39 => "🐨",
            MonsterKind::Mob40 => "🐨",
            MonsterKind::Mob41 => "🐨",
            MonsterKind::Mob42 => "🐨",
            MonsterKind::Mob43 => "🐨",
            MonsterKind::Mob44 => "🐨",
            MonsterKind::Mob45 => "🐨",
            MonsterKind::Mob46 => "🐨",
            MonsterKind::Mob47 => "🐨",
            MonsterKind::Mob48 => "🐨",
            MonsterKind::Mob49 => "🐨",
            MonsterKind::Mob50 => "🐨",
            MonsterKind::Boss01 => "🥦",
            MonsterKind::Boss02 => "🦋",
            MonsterKind::Boss03 => "🐞",
            MonsterKind::Boss04 => "🦁",
            MonsterKind::Boss05 => "🦝",
            MonsterKind::Boss06 => "🐮",
            MonsterKind::Boss07 => "🐯",
            MonsterKind::Boss08 => "🐼",
            MonsterKind::Boss09 => "🦍",
            MonsterKind::Boss10 => "🦖",
            MonsterKind::Boss11 => "🦚",
        }
    }

    pub fn display_name(&self) -> &'static str {
        ""
    }
}
