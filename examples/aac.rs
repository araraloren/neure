use neure::{bytes::BytesCtx, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digit = neure!(# [55 - 57]+);
    let mut ctx = BytesCtx::new("74".as_bytes());
    let mut storer = SpanStorer::default().with_capacity(1);

    dbg!(ctx.cap(0, &mut storer, &digit));
    dbg!(storer);

    Ok(())
}
