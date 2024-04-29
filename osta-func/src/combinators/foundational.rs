use osta_data::either::Either;
use crate::monads::fallible_state_monad::FallibleStateMonad;
use crate::monads::state_monad::StateMonad;

pub fn pair<'a, M1, M2, In: 'a, Out1: Copy + 'a, Out2: 'a>(
    left: M1,
    right: M2
) -> impl StateMonad<'a, In, (Out1, Out2)>
where
    M1: StateMonad<'a, In, Out1>,
    M2: StateMonad<'a, In, Out2>,
{
    left.and_then(move |out1| right.map(move |out2| (out1, out2)))
}

pub fn optional<'a, M, In: 'a, Out: 'a, Err: 'a>(monad: M) -> impl StateMonad<'a, In, Option<Out>>
where
    M: FallibleStateMonad<'a, In, Out, Err>,
{
    monad
        .map_out(Some)
        .or_else(move |_| move |input| (Ok(None), input))
        .map_out(|out| out.unwrap())
        .map(|result| result.unwrap_or_else(|_: ()| None))
}

pub fn composition<'a, M1, M2, In: 'a, Out1: Copy + 'a, Out2: 'a>(
    left: M1,
    right: M2
) -> impl StateMonad<'a, In, Either<Out1, Out2>>
    where
        M1: StateMonad<'a, In, Out1>,
        M2: StateMonad<'a, In, Option<Out2>>,
{
    left.and_then(
        move |left_out| right.map(move |right_out| match right_out {
            Some(right_out) => Either::Right(right_out),
            None => Either::Left(left_out),
        })
    )
}
