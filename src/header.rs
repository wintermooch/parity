use util::*;
use basic_types::*;

/// A block header.
///
/// Reflects the specific RLP fields of a block in the chain with additional room for the seal
/// which is non-specific.
///
/// Doesn't do all that much on its own.
#[derive(Debug)]
pub struct Header {
	pub parent_hash: H256,
	pub timestamp: U256,
	pub number: U256,
	pub author: Address,

	pub transactions_root: H256,
	pub uncles_hash: H256,
	pub extra_data: Bytes,

	pub state_root: H256,
	pub receipts_root: H256,
	pub log_bloom: LogBloom,
	pub gas_used: U256,
	pub gas_limit: U256,

	pub difficulty: U256,
	pub seal: Vec<Bytes>,

	pub hash: RefCell<Option<H256>>, //TODO: make this private
}

pub enum Seal {
	With,
	Without,
}

impl Header {
	/// Create a new, default-valued, header.
	pub fn new() -> Header {
		Header {
			parent_hash: ZERO_H256.clone(),
			timestamp: BAD_U256.clone(),
			number: ZERO_U256.clone(),
			author: ZERO_ADDRESS.clone(),

			transactions_root: ZERO_H256.clone(),
			uncles_hash: ZERO_H256.clone(),
			extra_data: vec![],

			state_root: ZERO_H256.clone(),
			receipts_root: ZERO_H256.clone(),
			log_bloom: ZERO_LOGBLOOM.clone(),
			gas_used: ZERO_U256.clone(),
			gas_limit: ZERO_U256.clone(),

			difficulty: ZERO_U256.clone(),
			seal: vec![],
			hash: RefCell::new(None),
		}
	}

	/// Get the hash of this header (sha3 of the RLP).
	pub fn hash(&self) -> H256 {
 		let mut hash = self.hash.borrow_mut();
 		match &mut *hash {
 			&mut Some(ref h) => h.clone(),
 			hash @ &mut None => {
 				*hash = Some(self.rlp_sha3(Seal::With));
 				hash.as_ref().unwrap().clone()
 			}
		}
	}

	/// Note that some fields have changed. Resets the memoised hash.
	pub fn note_dirty(&self) {
 		*self.hash.borrow_mut() = None;
	}

	// TODO: get hash without seal.

	// TODO: make these functions traity 
	pub fn stream_rlp(&self, s: &mut RlpStream, with_seal: Seal) {
		s.append_list(13 + match with_seal { Seal::With => self.seal.len(), _ => 0 });
		s.append(&self.parent_hash);
		s.append(&self.uncles_hash);
		s.append(&self.author);
		s.append(&self.state_root);
		s.append(&self.transactions_root);
		s.append(&self.receipts_root);
		s.append(&self.log_bloom);
		s.append(&self.difficulty);
		s.append(&self.number);
		s.append(&self.gas_limit);
		s.append(&self.gas_used);
		s.append(&self.timestamp);
		s.append(&self.extra_data);
		match with_seal {
			Seal::With => for b in self.seal.iter() { s.append_raw(&b, 1); },
			_ => {}
		}
	}

	pub fn rlp(&self, with_seal: Seal) -> Bytes {
		let mut s = RlpStream::new();
		self.stream_rlp(&mut s, with_seal);
		s.out()
	}

	pub fn rlp_sha3(&self, with_seal: Seal) -> H256 { self.rlp(with_seal).sha3() }
}

impl Decodable for Header {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		let d = try!(decoder.as_list());

		let mut blockheader = Header {
			parent_hash: try!(Decodable::decode(&d[0])),
			uncles_hash: try!(Decodable::decode(&d[1])),
			author: try!(Decodable::decode(&d[2])),
			state_root: try!(Decodable::decode(&d[3])),
			transactions_root: try!(Decodable::decode(&d[4])),
			receipts_root: try!(Decodable::decode(&d[5])),
			log_bloom: try!(Decodable::decode(&d[6])),
			difficulty: try!(Decodable::decode(&d[7])),
			number: try!(Decodable::decode(&d[8])),
			gas_limit: try!(Decodable::decode(&d[9])),
			gas_used: try!(Decodable::decode(&d[10])),
			timestamp: try!(Decodable::decode(&d[11])),
			extra_data: try!(Decodable::decode(&d[12])),
			seal: vec![],
			hash: RefCell::new(None),
		};

		for i in 13..d.len() {
			blockheader.seal.push(d[i].as_raw().to_vec());
		}

		Ok(blockheader)
	}
}

impl Encodable for Header {
	fn encode<E>(&self, encoder: &mut E) where E: Encoder {
		encoder.emit_list(| e | {
			self.parent_hash.encode(e);
			self.uncles_hash.encode(e);
			self.author.encode(e);
			self.state_root.encode(e);
			self.transactions_root.encode(e);
			self.receipts_root.encode(e);
			self.log_bloom.encode(e);
			self.difficulty.encode(e);
			self.number.encode(e);
			self.gas_limit.encode(e);
			self.gas_used.encode(e);
			self.timestamp.encode(e);
			self.extra_data.encode(e);
		
			for b in self.seal.iter() {
				e.emit_raw(&b);
			}
		})
	}
}
/*
trait RlpStandard {
	fn append(&self, s: &mut RlpStream);
}

impl RlpStandard for Header {
	fn append(&self, s: &mut RlpStream) {
		s.append_list(13);
		s.append(self.parent_hash);
		s.append_raw(self.seal[0]);
		s.append_standard(self.x);
	}
	fn populate(&mut self, s: &Rlp) {
	}
}

impl RlpStream {
	fn append_standard<O>(&mut self, o: &O) where O: RlpStandard {
		o.append(self);
	}
}
*/

#[cfg(test)]
mod tests {
}