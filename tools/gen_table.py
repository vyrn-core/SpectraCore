import json
import subprocess

with open('assets/mcrs/reports/packets.json') as f:
    report = json.load(f)

staged_text = subprocess.check_output(['git', 'show', ':crates/mcrs_minecraft_protocol/src/packets/table.rs']).decode()

table_typed = {}
current_table = None

for line in staged_text.splitlines():
    stripped = line.strip()
    if ' in ($crate::packets::' in stripped:
        tname = stripped.split(':')[0].strip()
        current_table = tname
        table_typed[current_table] = {}
    elif '=>' in stripped and current_table:
        parts = stripped.split('=>')
        k = parts[0].strip().strip('"')
        v = parts[1].strip().rstrip(',')
        table_typed[current_table][k] = v

# Adjustments for 1.21.11
if 'game_serverbound' in table_typed:
    table_typed['game_serverbound']['swing'] = 'ServerboundPunch'
    if 'attack' in table_typed['game_serverbound']:
        del table_typed['game_serverbound']['attack']

mapping = [
    ('handshake_serverbound', "ServerboundHandshakePacket<'a>", 'Handshaking', 'Serverbound', '($crate::packets::intent::serverbound)', 'handshake', 'serverbound'),
    ('status_clientbound', "ClientboundStatusPacket<'a>", 'Status', 'Clientbound', '($crate::packets::status::clientbound)', 'status', 'clientbound'),
    ('status_serverbound', 'ServerboundStatusPacket', 'Status', 'Serverbound', '($crate::packets::status::serverbound)', 'status', 'serverbound'),
    ('login_clientbound', "ClientboundLoginPacket<'a>", 'Login', 'Clientbound', '($crate::packets::login::clientbound)', 'login', 'clientbound'),
    ('login_serverbound', "ServerboundLoginPacket<'a>", 'Login', 'Serverbound', '($crate::packets::login::serverbound)', 'login', 'serverbound'),
    ('configuration_clientbound', "ClientboundConfigurationPacket<'a>", 'Configuration', 'Clientbound', '($crate::packets::configuration::clientbound)', 'configuration', 'clientbound'),
    ('configuration_serverbound', "ServerboundConfigurationPacket<'a>", 'Configuration', 'Serverbound', '($crate::packets::configuration::serverbound)', 'configuration', 'serverbound'),
    ('game_clientbound', "ClientboundGamePacket<'a>", 'Game', 'Clientbound', '($crate::packets::game::clientbound)', 'play', 'clientbound'),
    ('game_serverbound', "ServerboundGamePacket<'a>", 'Game', 'Serverbound', '($crate::packets::game::serverbound)', 'play', 'serverbound'),
]

out = []
out.append('''use crate::{ConnectionState, PacketSide};

pub const fn row_of(names: &[&str], name: &str) -> i32 {
    let mut row = 0;
    while row < names.len() {
        let (a, b) = (names[row].as_bytes(), name.as_bytes());
        if a.len() == b.len() {
            let mut i = 0;
            while i < a.len() && a[i] == b[i] {
                i += 1;
            }
            if i == a.len() {
                return row as i32;
            }
        }
        row += 1;
    }
    panic!("the name is not a row of its table")
}

#[derive(Debug, Clone, Copy)]
pub struct Table {
    pub name: &'static str,
    pub state: ConnectionState,
    pub side: PacketSide,
    pub names: &'static [&'static str],
    pub typed: &'static [&'static str],
}

#[derive(Debug)]
pub enum Decoded<P> {
    Packet(P),
    NotImplemented(&'static str),
    Unknown(i32),
}

impl<P> Decoded<P> {
    pub fn forget(self) -> Decoded<()> {
        match self {
            Decoded::Packet(_) => Decoded::Packet(()),
            Decoded::NotImplemented(name) => Decoded::NotImplemented(name),
            Decoded::Unknown(id) => Decoded::Unknown(id),
        }
    }
}

#[macro_export]
macro_rules! for_each_packet_table {
    ($callback:ident) => {
        $callback! {''')

for name, ty, st, side, mod_path, rep_st, rep_side in mapping:
    pkts = sorted(report[rep_st][rep_side].items(), key=lambda x: x[1]['protocol_id'])
    out.append(f'            {name}: {ty}, {st}, {side} in {mod_path} {{')
    typed_map = table_typed.get(name, {})
    for raw_pname, meta in pkts:
        pname = raw_pname.replace('minecraft:', '')
        if pname in typed_map:
            out.append(f'                "{pname}" => {typed_map[pname]},')
        else:
            out.append(f'                "{pname}",')
    out.append('            }')

out.append('''        }
    };
}

macro_rules! typed_name {
    ($name:literal, $ty:ident) => {
        $name
    };
}

macro_rules! tables {
    ($($table:ident: $enum:ident $(<$tlt:lifetime>)?, $state:ident, $side:ident in ($($module:tt)*) {
        $($name:literal $(=> $ty:ident $(<$lt:lifetime>)?)?,)*
    })*) => {
        $(
            pub mod $table {
                #[allow(unused_imports, unused_import_braces)]
                use $($module)*::{$($($ty,)?)*};

                pub const NAMES: &[&str] = &[$($name),*];
                pub const TYPED: &[&str] = &[$($(typed_name!($name, $ty),)?)*];
                pub const STATE: $crate::ConnectionState = $crate::ConnectionState::$state;
                pub const SIDE: $crate::PacketSide = $crate::PacketSide::$side;
                pub const TABLE: $crate::packets::table::Table = $crate::packets::table::Table {
                    name: stringify!($table),
                    state: STATE,
                    side: SIDE,
                    names: NAMES,
                    typed: TYPED,
                };

                $($(
                    impl $(<$lt>)? $crate::Packet for $ty $(<$lt>)? {
                        const ID: i32 = $crate::packets::table::row_of(NAMES, $name);
                        const NAME: &'static str = $name;
                        const SIDE: $crate::PacketSide = SIDE;
                        const STATE: $crate::ConnectionState = STATE;
                    }
                )?)*

                #[derive(Debug)]
                pub enum $enum $(<$tlt>)? {
                    $($($ty($ty $(<$lt>)?),)?)*
                }

                // chisle: a linear chain over this table's typed rows; the dispatcher
                // generated from the exported rows by another crate replaces it.
                pub fn decode$(<$tlt>)?(
                    id: i32,
                    r: &mut &$($tlt)? [u8],
                ) -> anyhow::Result<$crate::packets::table::Decoded<$enum $(<$tlt>)?>> {
                    $($(
                        if id == <$ty as $crate::Packet>::ID {
                            return <$ty as $crate::Decode>::decode(r)
                                .map($enum::$ty)
                                .map($crate::packets::table::Decoded::Packet)
                                .map_err(|error| {
                                    error.context(concat!("decoding ", $name, " (", stringify!($table), ")"))
                                });
                        }
                    )?)*
                    Ok(match usize::try_from(id).ok().and_then(|row| NAMES.get(row)) {
                        Some(name) => {
                            *r = &[];
                            $crate::packets::table::Decoded::NotImplemented(name)
                        }
                        None => $crate::packets::table::Decoded::Unknown(id),
                    })
                }
            }
        )*

        pub const TABLES: &[Table] = &[$($table::TABLE,)*];
    };
}

for_each_packet_table!(tables);
''')

with open('crates/mcrs_minecraft_protocol/src/packets/table.rs', 'w') as f:
    f.write('\n'.join(out))

print('table.rs rewritten cleanly!')
