/* Tiny-Tiny-Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

//! # 本模块的总则
//! 原则上讲，本模块应该至少遵循国际规范
//! 结构及其成员的命名应该遵循相关标准或论文中给定的标准论文（如果有）
//! 本模块可能在很长一段时间内都不会被实际使用，但可靠性应该被首要考虑
//! TODO: 在本模块的代码中插入对应标准或论文的链接的注释

#[derive(Debug)]
pub enum TLSError {
    RecodeTypeError(u8),
    RecodeVersionError(u8, u8),
    HandshakeContentTypeError(u8),
    UndefinedCiperSuite,
    BadRequest,
}

#[derive(Debug)]
pub enum RecodeType {
    ChangeCipherSpec,
    Alept,
    Handshake,
    ApplicationData,
}
impl RecodeType {
    fn new(byte: u8) -> Result<Self, TLSError> {
        match byte {
            20 => Ok(Self::ChangeCipherSpec),
            21 => Ok(Self::Alept),
            22 => Ok(Self::Handshake),
            23 => Ok(Self::ApplicationData),
            _ => Err(TLSError::RecodeTypeError(byte)),
        }
    }
}
#[derive(Debug)]
pub enum TLSVersion {
    SSL3_0,
    TLS1_0,
    TLS1_1,
    TLS1_2,
    TLS1_3,
}
impl TLSVersion {
    fn new(byte1: u8, byte2: u8) -> Result<Self, TLSError> {
        match (byte1, byte2) {
            (3, 0) => Ok(Self::SSL3_0),
            (3, 1) => Ok(Self::TLS1_0),
            (3, 2) => Ok(Self::TLS1_1),
            (3, 3) => Ok(Self::TLS1_2),
            (3, 4) => Ok(Self::TLS1_3),
            _ => Err(TLSError::RecodeVersionError(byte1, byte2)),
        }
    }
    fn bytes(self) -> (u8, u8) {
        match self {
            TLSVersion::SSL3_0 => (3, 0),
            TLSVersion::TLS1_0 => (3, 1),
            TLSVersion::TLS1_1 => (3, 2),
            TLSVersion::TLS1_2 => (3, 3),
            TLSVersion::TLS1_3 => (3, 4),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct RecordMessage {
    record_type: RecodeType,
    version: TLSVersion,
    pub length: u16,
}
impl RecordMessage {
    pub fn new(bytes: Vec<u8>) -> Result<Self, TLSError> {
        Ok(RecordMessage {
            record_type: RecodeType::new(bytes[0])?,
            version: TLSVersion::new(bytes[1], bytes[2])?,
            length: (bytes[3] as u16) << 8 | bytes[4] as u16,
        })
    }
}
macro_rules! build_ciper_suite {
    ($($e:ident=$v1:literal$(+$v2:literal)?),*) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub enum CipherSuite {
            $($e,)*
        }
        impl CipherSuite {
            fn new_2byte(type_v: u16) -> Result<Self, TLSError> {
                match type_v {
                    $($v1 => Ok(CipherSuite::$e),)*
                    _ => Err(TLSError::UndefinedCiperSuite)
                }
            }
            #[allow(dead_code)]
            fn new_1byte(type_v: u16) -> Result<Self, TLSError> {
                match type_v {
                    $($($v2 => Ok(CipherSuite::$e),)?)*
                    _ => Err(TLSError::UndefinedCiperSuite)
                }
            }
            fn into_u16(self) -> u16 {
                match self {
                    $(CipherSuite::$e => $v1,)*
                }
            }
            fn bytes(self) -> [u8; 2] {
                Self::into_u16(self).to_be_bytes()
            }
        }
    };
}
build_ciper_suite! {
    TLS_NULL_WITH_NULL_NULL = 0x0000 + 0x00,
    TLS_RSA_WITH_NULL_MD5 = 0x0001 + 0x01,
    TLS_RSA_WITH_NULL_SHA = 0x0002 + 0x02,
    TLS_RSA_EXPORT_WITH_RC4_40_MD5 = 0x0003 + 0x03,
    TLS_RSA_WITH_RC4_128_MD5 = 0x0004 + 0x04,
    TLS_RSA_WITH_RC4_128_SHA = 0x0005 + 0x05,
    TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5 = 0x0006 + 0x06,
    TLS_RSA_WITH_IDEA_CBC_SHA = 0x0007 + 0x07,
    TLS_RSA_EXPORT_WITH_DES40_CBC_SHA = 0x0008 + 0x08,
    TLS_RSA_WITH_DES_CBC_SHA = 0x0009 + 0x09,
    TLS_RSA_WITH_3DES_EDE_CBC_SHA = 0x000a + 0x0a,
    TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA = 0x000b + 0x0b,
    TLS_DH_DSS_WITH_DES_CBC_SHA = 0x000c + 0x0c,
    TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA = 0x000d + 0x0d,
    TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA = 0x000e + 0x0e,
    TLS_DH_RSA_WITH_DES_CBC_SHA = 0x000f + 0x0f,
    TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA = 0x0010 + 0x10,
    TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA = 0x0011 + 0x11,
    TLS_DHE_DSS_WITH_DES_CBC_SHA = 0x0012 + 0x12,
    TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA = 0x0013 + 0x13,
    TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA = 0x0014 + 0x14,
    TLS_AND_RSA_WITH_DES_CBC_SHA = 0x0015 + 0x15,
    TLS_AND_RSA_WITH_3DES_EDE_CBC_SHA = 0x0016 + 0x16,
    TLS_DH_anon_EXPORT_WITH_RC4_40_MD5 = 0x0017 + 0x17,
    TLS_DH_anon_WITH_RC4_128_MD5 = 0x0018 + 0x18,
    TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA = 0x0019+0x19,
    TLS_DH_anon_WITH_DES_CBC_SHA = 0x001a+0x1a,
    TLS_DH_anon_WITH_3DES_EDE_CBC_SHA = 0x001b+0x1b,
    // 0x00,0x1C-1D Reserved to avoid conflicts with SSLv3
    TLS_KRB5_WITH_DES_CBC_SHA = 0x001e+0x1e,
    TLS_KRB5_WITH_3DES_EDE_CBC_SHA = 0x001f+0x1f,
    TLS_KRB5_WITH_RC4_128_SHA = 0x0020+0x20,
    TLS_KRB5_WITH_IDEA_CBC_SHA = 0x0021+0x21,
    TLS_KRB5_WITH_DES_CBC_MD5 = 0x0022+0x22,
    TLS_KRB5_WITH_3DES_EDE_CBC_MD5 = 0x0023+0x23,
    TLS_KRB5_WITH_RC4_128_MD5 = 0x0024+0x24,
    TLS_KRB5_WITH_IDEA_CBC_MD5 = 0x0025+0x25,
    TLS_KRB5_EXPORT_WITH_DES_CBC_40_SHA = 0x0026+0x26,
    TLS_KRB5_EXPORT_WITH_RC2_CBC_40_SHA=0x0027+0x27,
    TLS_KRB5_EXPORT_WITH_RC4_40_SHA=0x0028+0x28,
    TLS_KRB5_EXPORT_WITH_DES_CBC_40_MD5=0x0029+0x29,
    TLS_KRB5_EXPORT_WITH_RC2_CBC_40_MD5=0x002a+0x2a,
    TLS_KRB5_EXPORT_WITH_RC4_40_MD5=0x002b+0x2b,
    TLS_PSK_WITH_NULL_SHA=0x002c+0x2c,
    TLS_DHE_PSK_WITH_NULL_SHA=0x002d+0x2d,
    TLS_RSA_PSK_WITH_NULL_SHA=0x002e+0x2e,
    TLS_RSA_WITH_AES_128_CBC_SHA = 0x002f + 0x2f,
    TLS_DH_DSS_WITH_AES_128_CBC_SHA = 0x0030 + 0x30,
    TLS_DH_RSA_WITH_AES_128_CBC_SHA = 0x0031 + 0x31,
    TLS_DHE_DSS_WITH_AES_128_CBC_SHA = 0x0032 + 0x32,
    TLS_AND_RSA_WITH_AES_128_CBC_SHA = 0x0033 + 0x33,
    TLS_DH_anon_WITH_AES_128_CBC_SHA = 0x0034+0x34,
    TLS_RSA_WITH_AES_256_CBC_SHA = 0x0035 + 0x35,
    TLS_DH_DSS_WITH_AES_256_CBC_SHA = 0x0036 + 0x36,
    TLS_DH_RSA_WITH_AES_256_CBC_SHA = 0x0037 + 0x37,
    TLS_DHE_DSS_WITH_AES_256_CBC_SHA = 0x0038 + 0x38,
    TLS_AND_RSA_WITH_AES_256_CBC_SHA = 0x0039 + 0x39,
    TLS_DH_anon_WITH_AES_256_CBC_SHA = 0x003a+0x3a,
    TLS_RSA_WITH_NULL_SHA256 = 0x003b + 0x3b,
    TLS_RSA_WITH_AES_128_CBC_SHA256 = 0x003c + 0x3c,
    TLS_RSA_WITH_AES_256_CBC_SHA256 = 0x003d + 0x3d,
    TLS_DH_DSS_WITH_AES_128_CBC_SHA256 = 0x003e + 0x3e,
    TLS_DH_RSA_WITH_AES_128_CBC_SHA256 = 0x003f + 0x3f,
    TLS_DHE_DSS_WITH_AES_128_CBC_SHA256 = 0x0040 + 0x40,
    TLS_RSA_WITH_CAMELLIA_128_CBC_SHA=0x0041+0x41,
    TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA=0x0042+0x42,
    TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA=0x0043+0x43,
    TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA=0x0044+0x44,
    TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA=0x0045+0x45,
    TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA=0x0046+0x46,
    //0x00,0x47-4F Reserved to avoid conflicts with deployed implementations
    //0x00,0x50-58 Reserved to avoid conflicts
    //0x00,0x59-5C Reserved to avoid conflicts with deployed implementations
    //0x00,0x5D-5F Unassigned
    //0x00,0x60-66 Reserved to avoid conflicts with widely deployed implementations
    TLS_DHE_RSA_WITH_AES_128_CBC_SHA256 = 0x0067 + 0x67,
    TLS_DH_DSS_WITH_AES_256_CBC_SHA256 = 0x0068 + 0x68,
    TLS_DH_RSA_WITH_AES_256_CBC_SHA256 = 0x0069 + 0x69,
    TLS_DHE_DSS_WITH_AES_256_CBC_SHA256 = 0x006a + 0x6a,
    TLS_DHE_RSA_WITH_AES_256_CBC_SHA256 = 0x006b + 0x6b,
    TLS_DH_anon_WITH_AES_128_CBC_SHA256 = 0x006c+0x6c,
    TLS_DH_anon_WITH_AES_256_CBC_SHA256=0x006d+0x6d,
    //0x00,0x6E-83 Unassigned
    TLS_RSA_WITH_CAMELLIA_256_CBC_SHA=0x0084+0x84,
    TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA=0x0085+0x85,
    TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA=0x0086+0x86,
    TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA=0x0087+0x87,
    TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA=0x0088+0x88,
    TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA=0x0089+0x89,
    TLS_PSK_WITH_RC4_128_SHA=0x008a+0x8a,
    TLS_PSK_WITH_3DES_EDE_CBC_SHA=0x008b+0x8b,
    TLS_PSK_WITH_AES_128_CBC_SHA=0x008c+0x8c,
    TLS_PSK_WITH_AES_256_CBC_SHA=0x008d+0x8d,
    TLS_DHE_PSK_WITH_RC4_128_SHA=0x008e+0x8e,
    TLS_DHE_PSK_WITH_3DES_EDE_CBC_SHA=0x008f+0x8f,
    TLS_DHE_PSK_WITH_AES_128_CBC_SHA=0x0090+0x90,
    TLS_DHE_PSK_WITH_AES_256_CBC_SHA=0x0091+0x91,
    TLS_RSA_PSK_WITH_RC4_128_SHA=0x0092+0x92,
    TLS_RSA_PSK_WITH_3DES_EDE_CBC_SHA=0x0093+0x93,
    TLS_RSA_PSK_WITH_AES_128_CBC_SHA=0x0094+0x94,
    TLS_RSA_PSK_WITH_AES_256_CBC_SHA=0x0095+0x95,
    TLS_RSA_WITH_SEED_CBC_SHA=0x0096+0x96,
    TLS_DH_DSS_WITH_SEED_CBC_SHA=0x0097+0x97,
    TLS_DH_RSA_WITH_SEED_CBC_SHA=0x0098+0x98,
    TLS_DHE_DSS_WITH_SEED_CBC_SHA=0x0099+0x99,
    TLS_DHE_RSA_WITH_SEED_CBC_SHA=0x009a+0x9a,
    TLS_DH_anon_WITH_SEED_CBC_SHA=0x009b+0x9b,
    TLS_RSA_WITH_AES_128_GCM_SHA256 = 0x009c + 0x9c,
    TLS_RSA_WITH_AES_256_GCM_SHA384 = 0x009d + 0x9d,
    TLS_DHE_RSA_WITH_AES_128_GCM_SHA256 = 0x009e + 0x9e,
    TLS_AND_RSA_WITH_AES_256_GCM_SHA384 = 0x009f + 0x9f,
    TLS_DH_RSA_WITH_AES_128_GCM_SHA256 = 0x00a0 + 0xa0,
    TLS_DH_RSA_WITH_AES_256_GCM_SHA384 = 0x00a1 + 0xa1,
    TLS_DHE_DSS_WITH_AES_128_GCM_SHA256 = 0x00a2 + 0xa2,
    TLS_DHE_DSS_WITH_AES_256_GCM_SHA384 = 0x00a3 + 0xa3,
    TLS_DH_DSS_WITH_AES_128_GCM_SHA256 = 0x00a4 + 0xa4,
    TLS_DH_DSS_WITH_AES_256_GCM_SHA384 = 0x00a5 + 0xa5,
    TLS_DH_anon_WITH_AES_128_GCM_SHA256 = 0x00a6+0xa6,
    TLS_DH_anon_WITH_AES_256_GCM_SHA384=0x00a7+0xa7,
    TLS_PSK_WITH_AES_128_GCM_SHA256=0x00a8+0xa8,
    TLS_PSK_WITH_AES_256_GCM_SHA384=0x00a9+0xa9,
    TLS_DHE_PSK_WITH_AES_128_GCM_SHA256=0x00aa+0xaa,
    TLS_DHE_PSK_WITH_AES_256_GCM_SHA384=0x00ab+0xab,
    TLS_RSA_PSK_WITH_AES_128_GCM_SHA256=0x00ac+0xac,
    TLS_RSA_PSK_WITH_AES_256_GCM_SHA384=0x00ad+0xad,
    TLS_PSK_WITH_AES_128_CBC_SHA256=0x00ae+0xae,
    TLS_PSK_WITH_AES_256_CBC_SHA384=0x00af+0xaf,
    TLS_PSK_WITH_NULL_SHA256=0x00b0+0xb0,
    TLS_PSK_WITH_NULL_SHA384=0x00b1+0xb1,
    TLS_DHE_PSK_WITH_AES_128_CBC_SHA256=0x00b2+0xb2,
    TLS_DHE_PSK_WITH_AES_256_CBC_SHA384=0x00b3+0xb3,
    TLS_DHE_PSK_WITH_NULL_SHA256=0x00b4+0xb4,
    TLS_DHE_PSK_WITH_NULL_SHA384=0x00b5+0xb5,
    TLS_RSA_PSK_WITH_AES_128_CBC_SHA256=0x00b6+0xb6,
    TLS_RSA_PSK_WITH_AES_256_CBC_SHA384=0x00b7+0xb7,
    TLS_RSA_PSK_WITH_NULL_SHA256=0x00b8+0xb8,
    TLS_RSA_PSK_WITH_NULL_SHA384=0x00b9+0xb9,
    TLS_RSA_WITH_CAMELLIA_128_CBC_SHA256=0x00ba+0xba,
    TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256=0x00bb+0xbb,
    TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256=0x00bc+0xbc,
    TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256=0x00bd+0xbd,
    TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256=0x00be+0xbe,
    TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA256=0x00bf+0xbf,
    TLS_RSA_WITH_CAMELLIA_256_CBC_SHA256=0x00c0+0xc0,
    TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256=0x00c1+0xc1,
    TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256=0x00c2+0xc2,
    TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256=0x00c3+0xc3,
    TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256=0x00c4+0xc4,
    TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA256=0x00c5+0xc5,
    //0x00,0xC6-FE         Unassigned
    TLS_EMPTY_RENEGOTIATION_INFO_SCSV=0x00ff+0xff,
    TLS_AES_128_GCM_SHA256 = 0x1301,
    TLS_AES_256_GCM_SHA384 = 0x1302,
    TLS_CHACHA20_POLY1305_SHA256 = 0x1303,
    TLS_ECDH_ECDSA_WITH_NULL_SHA = 0xc001,
    TLS_ECDH_ECDSA_WITH_RC4_128_SHA = 0xc002,
    TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA = 0xc003,
    TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA = 0xc004,
    TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA = 0xc005,
    TLS_ECDHE_ECDSA_WITH_NULL_SHA = 0xc006,
    TLS_ECDHE_ECDSA_WITH_RC4_128_SHA = 0xc007,
    TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA = 0xc008,
    TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA = 0xc009,
    TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA = 0xc00a,
    TLS_ECDH_RSA_WITH_NULL_SHA = 0xc00b,
    TLS_ECDH_RSA_WITH_RC4_128_SHA = 0xc00c,
    TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA = 0xc00d,
    TLS_ECDH_RSA_WITH_AES_128_CBC_SHA = 0xc00e,
    TLS_ECDH_RSA_WITH_AES_256_CBC_SHA = 0xc00f,
    TLS_ECDHE_RSA_WITH_NULL_SHA = 0xc010,
    TLS_ECDHE_RSA_WITH_RC4_128_SHA = 0xc011,
    TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA = 0xc012,
    TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA = 0xc013,
    TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA = 0xc014,
    TLS_ECDH_anon_WITH_NULL_SHA=0xc015,
    TLS_ECDH_anon_WITH_RC4_128_SHA=0xc016,
    TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA=0xc017,
    TLS_ECDH_anon_WITH_AES_128_CBC_SHA=0xc018,
    TLS_ECDH_anon_WITH_AES_256_CBC_SHA=0xc019,
    TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA=0xc01a,
    TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA=0xc01b,
    TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA=0xc01c,
    TLS_SRP_SHA_WITH_AES_128_CBC_SHA=0xc01d,
    TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA=0xc01e,
    TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA=0xc01f,
    TLS_SRP_SHA_WITH_AES_256_CBC_SHA=0xc020,
    TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA=0xc021,
    TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA=0xc022,
    TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256 = 0xc023,
    TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384 = 0xc024,
    TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256 = 0xc025,
    TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384 = 0xc026,
    TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256 = 0xc027,
    TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384 = 0xc028,
    TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256 = 0xc029,
    TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384 = 0xc02a,
    TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 = 0xc02b,
    TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 = 0xc02c,
    TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256 = 0xc02d,
    TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384 = 0xc02e,
    TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 = 0xc02f,
    TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 = 0xc030,
    TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256 = 0xc031,
    TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384 = 0xc032,
    TLS_ECDHE_PSK_WITH_RC4_128_SHA=0xc033,
    TLS_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA=0xc034,
    TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA=0xc035,
    TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA =0xc036,
    TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256=0xc037,
    TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384=0xc038,
    TLS_ECDHE_PSK_WITH_NULL_SHA=0xc039,
    TLS_ECDHE_PSK_WITH_NULL_SHA256 =0xc03a,
    TLS_ECDHE_PSK_WITH_NULL_SHA384=0xc03b,
    TLS_ECDHE_EDDSA_WITH_CHACHA20_POLY1305=0xccb0,
    TLS_ECDHE_EDDSA_WITH_AES_128_GCM_SHA256=0xccb1,
    TLS_ECDHE_EDDSA_WITH_AES_256_GCM_SHA256=0xccb2
    //0xC0,0x3C-FF Unassigned
    //0xC1-FD,*  Unassigned
    //0xFE,0x00-FD Unassigned
    //0xFE,0xFE-FF Reserved to avoid conflicts with widely deployed implementations
    //0xFF,0x00-FF Reserved for Private Use
}
#[derive(Debug)]
pub enum HandshakeContent {
    HelloRequest,
    ClientHello(HandshakeClientHello),
    ServerHello(HandshakeServerHello),
    Certificate(HandshakeCertificate),
    ServerKeyExchange(HandshakeServerKeyExchange),
    CertificateRequest,
    HelloDone,
    CertificateVerify,
    ClientKeyExchange,
    Finished,
}
#[derive(Debug)]
pub enum CompressionMethod {
    Null,
    Deflate,
    Undefined,
}
impl CompressionMethod {
    fn new(byte: u8) -> Self {
        match byte {
            0 => Self::Null,
            1 => Self::Deflate,
            _ => Self::Undefined,
        }
    }
    fn into_u8(self) -> u8 {
        match self {
            Self::Deflate => 1,
            _ => 0,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Random {
    pub timestamp: u32,
    pub random_bytes: [u8; 28],
}
impl Random {
    pub fn new_32bit_random(random_bytes: [u8; 32]) -> Self {
        Random {
            timestamp: u32::from_be_bytes(random_bytes[0..4].try_into().unwrap()),
            random_bytes: random_bytes[4..].try_into().unwrap(),
        }
    }
    pub fn bytes(self) -> [u8; 32] {
        let mut vec = self.timestamp.to_be_bytes().to_vec();
        vec.extend(self.random_bytes);
        vec.try_into().unwrap()
    }
}
macro_rules! build_tls_extension {
    ($($e:ident=$v:literal),*) => {
        #[derive(Debug)]
        #[allow(dead_code)]
        pub enum TLSExtension {
            $($e,)*
        }
        impl TLSExtension {
            #[allow(dead_code)]
            fn new(_v: u16) -> Self {
                match _v {
                    $($v => TLSExtension::$e,)*
                    _ => TLSExtension::Undefined
                }
            }
        }
    };
}
build_tls_extension! {
    ServerName=0,
    MaxFragmentLength=1,
    ClientCertificateURL=2,
    TrustedCaKeys=3,
    TruncatedHMAC=4,
    StatusRequest=5,
    UserMapping=6,
    ClientAuthz=7,
    ServerAuthz=8,
    CertType=9,
    SupportedGroups=10,
    ECPointFormats=11,
    Srp=12,
    SignatureAlgorithms=13,
    UseSRTP=14,
    Heartbeat=15,
    ApplicationLayerProtocolNegotiation=16,
    StatusRequestV2=17,
    SignedCertificateTimestamp=18,
    ClientCertificateType=19,
    ServerCertificateType=20,
    Padding=21,
    EncryptThenMAC=22,
    ExtendedMasterSecret=23,
    TokenBingding=24,
    CachedInfo=25,
    TlsLts=26,
    CompressCertificate=27,
    RecordSizeLimit=28,
    PwdProtect=29,
    PwdClear=30,
    PasswordSalt=31,
    TicketPinning=32,
    TlsVertWithExternPsk=33,
    DelegatedCredential=34,
    SessionTicket=35,
    Tlmsp=36,
    TlmspProxying=37,
    TlmspDelegate=38,
    SupportedEktCiphers=39,
    PreSharedKey=41,
    EarlyData=42,
    SupportedVersions=43,
    Cookie=44,
    PskKeyExchangeModes=45,
    CertificateAuthorities=47,
    OidFilters=48,
    PostHandshakeAuth=49,
    SignatureAlgorithmsCert=50,
    KeyShare=51,
    TransparencyInfo=52,
    ConnectionIdDeprecated=53,
    ConnectionId=54,
    ExternalIdHash=55,
    ExternalSessionId=56,
    QuicTransportParameters=57,
    TicketRequest=58,
    DnssecChain=59,
    SequenceNumberEncryptionAlgorithms=60,
    Rrc=61,
    Undefined=65535
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct HandshakeCertificate {
    pub certificate_length: u32,
    pub certificates: Vec<Vec<u8>>,
}
impl HandshakeCertificate {
    pub fn new_just_one_certificate(certificate: Vec<u8>) -> Self {
        let mut vec = vec![];
        let length_snd: u32 = certificate.len() as u32;
        let length_fst: u32 = length_snd + 3;
        vec.extend(&length_snd.to_be_bytes()[1..]);
        vec.extend(certificate);
        HandshakeCertificate {
            certificate_length: length_fst,
            certificates: vec![vec],
        }
    }
    pub fn bytes(self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(&self.certificate_length.to_be_bytes()[1..]);
        for e in self.certificates {
            vec.extend(e);
        }
        vec
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct HandshakeClientHello {
    pub version: TLSVersion,
    pub random: Random,
    pub session_id: Option<Vec<u8>>,
    pub ciper_suites: Vec<CipherSuite>,
    pub compression_method: CompressionMethod,
    pub extenssions_length: u16,
}
impl HandshakeClientHello {
    fn new(mut bytes: Vec<u8>) -> Result<Self, TLSError> {
        let version = TLSVersion::new(bytes.remove(0), bytes.remove(0))?;
        let random = Random {
            timestamp: (bytes.remove(0) as u32) << 24
                | ((bytes.remove(0) as u32) << 16)
                | ((bytes.remove(0) as u32) << 8)
                | (bytes.remove(0) as u32),
            random_bytes: if let Ok(a) = bytes.drain(0..28).as_slice().try_into() {
                a
            } else {
                return Err(TLSError::BadRequest);
            },
        };
        let session_id_length = bytes.remove(0);
        let session_id_ori = bytes.drain(0..(session_id_length as usize)).collect();
        let session_id = if session_id_ori == vec![] {
            None
        } else {
            Some(session_id_ori)
        };
        let mut ciper_suites_length = (bytes.remove(0) as u16) << 8 | (bytes.remove(0) as u16);
        let mut ciper_suites = vec![];
        while ciper_suites_length != 0 {
            if let Ok(suite) =
                CipherSuite::new_2byte((bytes.remove(0) as u16) << 8 | (bytes.remove(0) as u16))
            {
                ciper_suites.push(suite)
            }
            ciper_suites_length -= 2;
        }
        let compression_method = CompressionMethod::new(bytes[0]);
        let extenssions_length = (bytes.remove(0) as u16) << 8 | (bytes.remove(0) as u16);
        Ok(HandshakeClientHello {
            version,
            random,
            session_id,
            ciper_suites,
            compression_method,
            extenssions_length,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HandshakeServerHello {
    pub version: TLSVersion,
    pub random: Random,
    pub session_id: Option<Vec<u8>>,
    pub ciper_suite: CipherSuite,
    pub compression_method: CompressionMethod,
    pub extenssions_length: u16,
}
impl HandshakeServerHello {
    fn new(mut bytes: Vec<u8>) -> Result<Self, TLSError> {
        let version = TLSVersion::new(bytes.remove(0), bytes.remove(0))?;
        let random = Random {
            timestamp: (bytes.remove(0) as u32) << 24
                | ((bytes.remove(0) as u32) << 16)
                | ((bytes.remove(0) as u32) << 8)
                | (bytes.remove(0) as u32),
            random_bytes: if let Ok(a) = bytes.drain(0..28).as_slice().try_into() {
                a
            } else {
                return Err(TLSError::BadRequest);
            },
        };
        let session_id_length = bytes.remove(0);
        let session_id_ori = bytes.drain(0..(session_id_length as usize)).collect();
        let session_id = if session_id_ori == vec![] {
            None
        } else {
            Some(session_id_ori)
        };
        let ciper_suite =
            CipherSuite::new_2byte((bytes.remove(0) as u16) << 8 | (bytes.remove(0) as u16))?;
        let compression_method = CompressionMethod::new(bytes[0]);
        let extenssions_length = (bytes.remove(0) as u16) << 8 | (bytes.remove(0) as u16);
        Ok(HandshakeServerHello {
            version,
            random,
            session_id,
            ciper_suite,
            compression_method,
            extenssions_length,
        })
    }
    pub fn bytes(self) -> Vec<u8> {
        let mut bytes = vec![];
        let (version_byte1, version_byte2) = self.version.bytes();
        bytes.push(version_byte1);
        bytes.push(version_byte2);
        bytes.extend(self.random.bytes());
        if let Some(id) = self.session_id {
            bytes.push(id.len().try_into().unwrap());
            bytes.extend(id);
        } else {
            bytes.push(0);
        }
        bytes.extend(self.ciper_suite.bytes());
        bytes.push(self.compression_method.into_u8());

        bytes.push(0);
        bytes.push(5);

        bytes.extend([0xff, 0x01, 0x00, 0x01, 0x00]); // temp: RenegotiationInfo

        bytes
    }
}
#[derive(Debug)]
pub enum CurveName {
    X25519,
}
impl CurveName {
    fn bytes(self) -> (u8, u8) {
        match self {
            Self::X25519 => (0, 0x001d),
        }
    }
}
#[derive(Debug)]
pub struct HandshakeServerKeyExchange {
    pub curve_name: CurveName,
    pub public_key: Vec<u8>,
    pub sign: Vec<u8>,
}
impl HandshakeServerKeyExchange {
    pub fn bytes(self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(3);
        let curve_name_bytes = self.curve_name.bytes();
        vec.push(curve_name_bytes.0);
        vec.push(curve_name_bytes.1);
        vec.push(self.public_key.len() as u8);
        vec.extend(self.public_key);
        // 0503: secp256r1 曲线
        // 0020: 32 字节
        vec.extend([0x05, 0x03, 0x00, 0x20]);
        vec.extend(self.sign);
        vec
    }
}

impl HandshakeContent {
    fn new(mut bytes: Vec<u8>) -> Result<Self, TLSError> {
        match bytes.remove(0) {
            0 => Ok(HandshakeContent::HelloRequest),
            1 => Ok(HandshakeContent::ClientHello(HandshakeClientHello::new(
                bytes,
            )?)),
            2 => Ok(HandshakeContent::ServerHello(HandshakeServerHello::new(
                bytes,
            )?)),
            11 => todo!(),
            12 => todo!(),
            13 => Ok(HandshakeContent::CertificateRequest),
            14 => Ok(HandshakeContent::HelloDone),
            15 => Ok(HandshakeContent::CertificateVerify),
            16 => Ok(HandshakeContent::ClientKeyExchange),
            20 => Ok(HandshakeContent::Finished),
            _ => Err(TLSError::HandshakeContentTypeError(bytes[0])),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HandshakeMessage {
    pub handshake_content: HandshakeContent,
    pub length: u32,
}
impl HandshakeMessage {
    fn new(mut bytes: Vec<u8>) -> Result<Self, TLSError> {
        let length = (bytes.remove(1) as u32) << 16
            | ((bytes.remove(1) as u32) << 8)
            | (bytes.remove(1) as u32);
        Ok(HandshakeMessage {
            length,
            handshake_content: HandshakeContent::new(bytes)?,
        })
    }
    pub fn bytes_without_length(self) -> Vec<u8> {
        match self.handshake_content {
            HandshakeContent::HelloRequest => todo!(),
            HandshakeContent::ClientHello(_) => todo!(),
            HandshakeContent::ServerHello(a) => {
                let mut vec = vec![];
                vec.push(2);
                let bytes = a.bytes();
                vec.extend(&(bytes.len() as u32).to_be_bytes()[1..]);
                vec.extend(bytes);
                vec
            }
            HandshakeContent::Certificate(a) => {
                let mut vec = vec![];
                vec.push(0x0b);
                let bytes = a.bytes();
                vec.extend(&(bytes.len() as u32).to_be_bytes()[1..]);
                vec.extend(bytes);
                vec
            }
            HandshakeContent::ServerKeyExchange(a) => {
                let mut vec = vec![];
                vec.push(0x0c);
                let bytes = a.bytes();
                vec.extend(&(bytes.len() as u32).to_be_bytes()[1..]);
                vec.extend(bytes);
                vec
            }
            HandshakeContent::CertificateRequest => todo!(),
            HandshakeContent::HelloDone => [0x0e, 0, 0, 0].to_vec(),
            HandshakeContent::CertificateVerify => todo!(),
            HandshakeContent::ClientKeyExchange => todo!(),
            HandshakeContent::Finished => todo!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TLSMessage {
    pub record_message: RecordMessage,
    pub handshake_message: HandshakeMessage,
}
#[allow(dead_code)]
pub fn parse(mut data: Vec<u8>) -> Result<TLSMessage, TLSError> {
    Ok(TLSMessage {
        record_message: RecordMessage::new(data.drain(0..5).collect())?,
        handshake_message: HandshakeMessage::new(data)?,
    })
}
pub fn parse_has_record(
    record_message: RecordMessage,
    extra: Vec<u8>,
) -> Result<TLSMessage, TLSError> {
    Ok(TLSMessage {
        record_message,
        handshake_message: HandshakeMessage::new(extra)?,
    })
}

pub fn get_server_record_tls1_2_bytes(length: u16) -> Vec<u8> {
    let mut vec = Vec::from([0x16, 0x03, 0x03]);
    vec.extend(length.to_be_bytes());
    vec
}
