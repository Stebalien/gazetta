pub trait BubbleResult {
    type Value;
    type Error;
    fn bubble_result(self) -> Result<Self::Value, Self::Error>;
}

impl<V, E> BubbleResult for Result<V, E> {
    type Value = V;
    type Error = E;
    fn bubble_result(self) -> Self {
        self
    }
}

impl<I> BubbleResult for Option<I>
    where I: BubbleResult
{
    type Value = Option<I::Value>;
    type Error = I::Error;

    fn bubble_result(self) -> Result<Self::Value, Self::Error> {
        match self {
            Some(v) => match v.bubble_result() {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            },
            None => Ok(None),
        }
    }
}

