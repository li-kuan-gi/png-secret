mod chunk_type;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
