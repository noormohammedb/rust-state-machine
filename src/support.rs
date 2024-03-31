pub struct Block<Header, Extrinsic> {
	pub header: Header,
	pub extrinsics: Vec<Extrinsic>,
}

pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

pub type DispatchResult = core::result::Result<(), &'static str>;

pub trait Dispatch {
	type Caller;
	type Call;
	fn dispatch(&mut self, caller: Self::Call, call: Self::Call) -> DispatchResult;
}