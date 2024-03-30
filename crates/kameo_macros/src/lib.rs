mod actor;
mod derive_actor;

use actor::Actor;
use derive_actor::DeriveActor;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

/// Attribute macro placed on `impl` blocks of actors to define messages.
///
/// Methods on the impl block are marked with `#[message]` or `#[query]`.
/// This generates a struct for the message, allowing it to be sent to the actor.
///
/// # Example
///
/// ```
/// use kameo::actor;
///
/// #[actor]
/// impl Counter {
///     /// Regular message
///     #[message]
///     pub fn inc(&mut self, amount: u32) -> Result<i64, Infallible> {
///         self.count += amount as i64;
///         Ok(self.count)
///     }
///
///     /// Regular query
///     #[query]
///     pub fn count(&self) -> Result<i64, Infallible> {
///         Ok(self.count)
///     }
///
///     /// Derives on the message
///     #[message(derive(Clone, Copy))]
///     pub fn dec(&self, amount: u32) {
///         self.count -= amount as i64;
///     }
/// }
///
/// counter_ref.send(Inc { amount: 5 }).await??;
/// counter_ref.query(Count).await??;
/// counter_ref.send(Dec { amount: 2 }.clone()).await??;
/// ```
///
/// <details>
/// <summary>See expanded code</summary>
///
/// ```
/// pub struct Inc {
///     pub amount: u32,
/// }
///
/// impl kameo::Message<Counter> for Inc {
///     type Reply = Result<i64, Infallible>;
///
///     async fn handle(self, state: &mut Counter) -> Self::Reply {
///         state.inc(self.amount)
///     }
/// }
///
/// pub struct Count;
///
/// impl kameo::Query<Counter> for Count {
///     type Reply = Result<i64, Infallible>;
///
///     async fn handle(self, state: &Counter) -> Self::Reply {
///         state.count()
///     }
/// }
///
/// #[derive(Clone, Copy)]
/// pub struct Dec;
///
/// impl kameo::Message<Counter> for Dec {
///     type Reply = Result<(), Infallible>;
///
///     async fn handle(self, state: &mut Counter) -> Self::Reply {
///         state.dec(self.amount)
///     }
/// }
/// ```
/// </details>
#[proc_macro_attribute]
pub fn actor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let actor = parse_macro_input!(item as Actor);
    TokenStream::from(actor.into_token_stream())
}

/// Derive macro implementing the [Actor](https://docs.rs/kameo/latest/kameo/trait.Actor.html) trait with default behaviour.
///
/// The [Actor::name](https://docs.rs/kameo/latest/kameo/trait.Actor.html#method.name) is implemented using the actor's ident.
///
/// # Example
///
/// ```
/// use kameo::Actor;
///
/// #[derive(Actor)]
/// struct MyActor { }
///
/// assert_eq!(MyActor { }.name(), "MyActor");
/// ```
#[proc_macro_derive(Actor)]
pub fn derive_actor(input: TokenStream) -> TokenStream {
    let derive_actor = parse_macro_input!(input as DeriveActor);
    TokenStream::from(derive_actor.into_token_stream())
}