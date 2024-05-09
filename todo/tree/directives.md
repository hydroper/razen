# Directives

## Defer

* [ ] Statements are verified only after directives, in two different verification methods (one verification method for directives, and one pass verification method for statements). Block statements, with the right scopes, are entered recursively for directives.
* [ ] Directives are always have a cache to prevent re-verification using the node mapping of SemanticHost; it may just be an invalidation thingy when it does not matter, such as for an use namespace directive.
* [ ] When a directive throws a defer error, the entire verification should reoccur next time.
* [ ] Addition: the former explanations should be expanded such that deferred verification occurs in compilation unit level.

## Variable definitions

* [ ] Assign ASDoc to the first topmost variable binding's slot after full resolution.
* [ ] Assign meta-data to the first variable binding's slot after full resolution.
* [ ] Assign `[Bindable]` semantics to topmost variable slots after full resolution.

## Class definitions

* [ ] Assign ASDoc
* [ ] Assign location
* [ ] Assign every `[Event]` semantics to the class
* [ ] Mark unused

## Function definitions

Function definitions should have careful plannings. It involves caching the activation, setting phases (similiarly to destructuring), and avoiding verifying things twice (the signature, that is). They should also be able to do inference in the signature's result type depending on the `inferTypes` option.

Never ever let getters and setters have the wrong signature assigned to them; if they are invalid, just use a default signature matching their requirements.

Handle conflicting definitions properly, only moving forward in verification if the resulting slot is a method slot and not something else (it could be a variable slot or a class, for example).