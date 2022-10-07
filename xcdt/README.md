# eXtensible Chained Data Type

Brings Chaos into Rust. Well, just for fun.

## About

xcdt is a lib that implements inheritance and virtual methods features in Rust. All derived types can automatically inherit parent's behaviors, and can even overwrite it (i.e, virutal methods).

xcdt depends on a nightly feature `generic_associated_types`, and if you need virtual methods, another feature `min_specialization` is required.

## Inherit Data Members

To inherit Data Memebrs, call the following macro:

```rust
xcdt::declare_xcdt!(CoreNode, NodeProps, Object, ObjectBase);

pub struct NodeProps {
    // ... your data members in derived type
}

```

The above call to `declare_xcdt` will do the following things for you:

- Create a Type named `CoreNode`, which is the derived type.
- Data member of `CoreNode` is defined as `NodeProps`, in which you can add your own data members.
- `CoreNode` will inherit data and behaviors from `Object`. Object is the base type for all the xcdt types.
- Well, the last `ObjectBase` is a bit duplicated with the third parameter `Object`. Maybe we can refine the macro definition a little bit to eliminate this redundant param, though I'm not sure.

Now, CoreNode will be the type containing both data from base type and derived type. Let's see how to make an instance of CoreNode.

## Use Builder to create instances

xcdt provides a builder for each xcdt type. For example,

```rust
CoreNode::builder()
     .with(NodeProps::new(...))
     .build()
```

And when the inheritance chain gets longer:

```rust
CoreHtmlElement::builder()
    .with(NodeProps::new(...))
    .with(ElementProps::new(...))
    .with(HtmlElementProps::new(...)
    .build()
```

## Define behaviors

The xcdt data type (e.g., `CoreNode`) doesn't expose any behavours publicly. We need to use trais to implement behaviors and get it exposed to the public. Let's say, we have an `INode` trait:

```rust
pub trait INode {
    fn test(&self);
}
```

Now let's implement the trait. If you want to make it inheritable to all the dervied types from CoreNode, implement the trait for `CoreNodeBase<T>`:

```rust
impl<T: 'static + XcDataType> INode for CoreNodeBase<T> {
    fn test(&self) {}
}
```

Then all the dervied types will automatically implement the same trait, which is just like what other OO-languages do.

Or, if you don't want it to be inherited by derived types, implement the trait for `CoreNode`:

```rust
impl INode for CoreNode {
    fn test(&self) {}
}
```

Then the method will be only implemented on CoreNode, rather than all of its dervied types.

## Access data members

Data members can be accessed in Trait implementations with `self.<PropName>`. For example,

```rust
xcdt::declare_xcdt!(CoreNode, NodeProps, Object, ObjectBase);

pub struct NodeProps {
    // ... your data members in derived type
}

impl INode for CoreNode {
    fn test(&self) {
        self.NodeProps() // <- this will return the instance of NodeProps
    }
}

```

You can use the same way to access parent's data member. Assume that we have a new type called `CoreElement` which is a derived class of `CoreNode`, we can access the props of `CoreNode` just in the same way:

```rust
xcdt::declare_xcdt!(CoreElement, ElementProps, CoreNode, CoreNodeBase);

pub struct ElementProps {
    // ... your data members in derived type
}

impl IElement for CoreElement {
    fn test(&self) {
        self.NodeProps() // <- this will return the instance of NodeProps
        self.ElementProps() // <- this will return the instance of ElementProps
    }
}
```

## Virtual Methods

Virtual methods are achived by using the nightly feature `min_specialization`, which provides template specialization.

Just mark the implementation with `default`, then we will have a chance to overwrite it in our derived types:

```rust
impl<T: 'static + XcDataType> Layoutable for CoreNodeBase<T> {
    default fn layout(&self) {
        // ... Implementation in Base type
    }
}

// In derived types..
impl<T: 'static + XcDataType> Layoutable for CoreElementBase<T> {
    default fn layout(&self) {
        // ... Override parent's implementation
    }
}
```

Any call to `layout` will be dispatched to the correct implementation, according to what the concrete data type is.
