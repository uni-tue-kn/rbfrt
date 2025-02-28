/* Copyright 2023-present University of Tuebingen, Chair of Communication Networks
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * Steffen Lindner (steffen.lindner@uni-tuebingen.de)
 */

use crate::bfrt::Convert;
use crate::error::RBFRTError;
use std::array::TryFromSliceError;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Converts internal data representation to vector of bytes
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
    fn to_u32(&self) -> u32 {
        unimplemented!("Conversion not implemented.");
    }

    fn to_u64(&self) -> u64 {
        unimplemented!("Conversion not implemented.");
    }
    fn to_u128(&self) -> u128 {
        unimplemented!("Conversion not implemented.");
    }
    fn to_string(&self) -> String {
        unimplemented!("Conversion not implemented.");
    }
    fn to_bool(&self) -> bool {
        unimplemented!("Conversion not implemented.");
    }
    fn to_ipv4(&self) -> Result<Ipv4Addr, RBFRTError> {
        unimplemented!("Conversion not implemented.");
    }
    fn to_ipv6(&self) -> Result<Ipv6Addr, RBFRTError> {
        unimplemented!("Conversion not implemented.");
    }
    fn to_int_arr(&self) -> Vec<u32> {
        unimplemented!("Conversion not implemented");
    }
}

impl ToBytes for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        u8::to_be_bytes(*self).to_vec()
    }
}

impl ToBytes for i8 {
    fn to_bytes(&self) -> Vec<u8> {
        i8::to_be_bytes(*self).to_vec()
    }
}

impl ToBytes for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        u16::to_be_bytes(*self).to_vec()
    }
}

impl ToBytes for i16 {
    fn to_bytes(&self) -> Vec<u8> {
        i16::to_be_bytes(*self).to_vec()
    }
}

impl ToBytes for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        u32::to_be_bytes(*self).to_vec()
    }
}

impl ToBytes for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        i32::to_be_bytes(*self).to_vec()
    }
}

impl ToBytes for bool {
    fn to_bytes(&self) -> Vec<u8> {
        if *self {
            vec![1]
        } else {
            vec![0]
        }
    }
}

impl ToBytes for Vec<u8> {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn to_u32(&self) -> u32 {
        let data = self.clone().convert("to_u32 call", 32).unwrap();
        u32::from_be_bytes(data.try_into().unwrap())
    }

    fn to_u64(&self) -> u64 {
        let data = self.clone().convert("to_u64 call", 64).unwrap();
        u64::from_be_bytes(data.try_into().unwrap())
    }

    fn to_u128(&self) -> u128 {
        let data = self.clone().convert("to_u128 call", 128).unwrap();
        u128::from_be_bytes(data.try_into().unwrap())
    }

    fn to_string(&self) -> String {
        std::str::from_utf8(self).unwrap().to_string()
    }

    fn to_bool(&self) -> bool {
        self.iter().any(|&x| x > 0u8)
    }

    /// Converts `Vec<u8>` of length 4 to `Ipv4Add`.
    ///
    /// # Errors
    ///
    /// Throws an `RBFRTError` if vector length does not match.
    ///
    /// # Example
    ///
    ///```
    /// use std::net::Ipv4Addr;
    /// use rbfrt::table::ToBytes;
    ///
    /// let ip = vec![192u8,168,0,1].to_ipv4().unwrap();
    /// assert_eq!(ip.octets().to_vec(), vec![192,168,0,1])
    ///```
    fn to_ipv4(&self) -> Result<Ipv4Addr, RBFRTError> {
        let octets: [u8; 4] = self[..].try_into().map_err(|e: TryFromSliceError| {
            RBFRTError::ByteConversionError {
                target: "Ipv4Addr".to_owned(),
                orig_e: e.into(),
            }
        })?;
        Ok(Ipv4Addr::from(octets))
    }

    fn to_int_arr(&self) -> Vec<u32> {
        let mut ret = vec![];
        for (i, v) in self.iter().enumerate().step_by(4) {
            // TODO dont do unsafe ...
            ret.push(unsafe {
                u32::from_be_bytes([
                    *v,
                    *self.get_unchecked(i + 1),
                    *self.get_unchecked(i + 2),
                    *self.get_unchecked(i + 3),
                ])
            });
        }

        ret
    }

    /// Converts `Vec<u8>` of length 4 to `Ipv4Add`.
    ///
    /// # Errors
    ///
    /// Throws an `RBFRTError` if vector length does not match.
    ///
    /// # Example
    ///
    ///```
    /// use std::net::Ipv6Addr;
    /// use rbfrt::table::ToBytes;
    ///
    /// let ip = vec![255u8,255,1,2,3,4,5,6,7,8,9,10,11,12,255,1].to_ipv6().unwrap();
    /// assert_eq!(ip.octets().to_vec(), vec![255u8,255,1,2,3,4,5,6,7,8,9,10,11,12,255,1])
    ///```
    fn to_ipv6(&self) -> Result<Ipv6Addr, RBFRTError> {
        // convert Vec<u8> to [u8; 16]
        let octets: [u8; 16] = self[..].try_into().map_err(|e: TryFromSliceError| {
            RBFRTError::ByteConversionError {
                target: "Ipv6Addr".to_owned(),
                orig_e: e.into(),
            }
        })?;

        // Conversion passes
        Ok(Ipv6Addr::from(octets))
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for &str {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for Ipv4Addr {
    /// Converts an Ipv4Addr Object to`Vec<u8>` of length 4
    ///```
    /// use std::net::Ipv4Addr;
    /// use rbfrt::table::ToBytes;
    /// let data = Ipv4Addr::from([10,0,0,2]).to_bytes();
    /// assert_eq!(data, vec![10,0,0,2])
    ///```
    fn to_bytes(&self) -> Vec<u8> {
        self.octets().to_vec()
    }
}

impl ToBytes for Ipv6Addr {
    /// Converts an Ipv6Addr Object to `Vec<u8>` of length 16
    ///```
    /// use std::net::Ipv6Addr;
    /// use rbfrt::table::ToBytes;
    /// let data = Ipv6Addr::from([255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]).to_bytes();
    /// assert_eq!(data, vec![255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1])
    ///```
    fn to_bytes(&self) -> Vec<u8> {
        self.octets().to_vec()
    }
}

impl ToBytes for Vec<u32> {
    fn to_bytes(&self) -> Vec<u8> {
        self.iter().flat_map(|val| val.to_be_bytes()).collect()
    }
}
