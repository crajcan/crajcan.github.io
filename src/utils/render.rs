// Flow would be like, component has something happen that triggers "re-render" process
// It calls render from the Render trait, which in turn calls as_element(), which the component
// provides via another required method.

// Then it attaches it with the pointer that the component provides via another required method

// This feels like dogshit

// so we were talking about render just creating a string....

// That might have different implications for my 'flow' above
// Flow would be some function is attached to an html element, then onClick happens
// onClick calls a trait method on the trait Attach or something, which expects render to exist
// on the component struct

// other open question is if we can attach a function (like onclick) to the DOM without exporting
// that handler as a JS function.

// I have an example of that too:

// #[wasm_bindgen]
// pub fn add(a: u32, b: u32) -> u32 {
//     a + b
// }

pub trait Attach {
    // problem with render just returning a String is that it needs to re-render the children too
    // I'm not sure if I can use web-sys to attach a massive nested html string.

    // This is where I was just researching how React does it
    // If each component just has a method that can create an Element...
    // like as_element() or something
    // Then each component's as_element() can just call that on the children I think
    // problem is no virtual dom
    // plan was for each component struct just to hold a reference to its actual rendered element in the dom
    // then it can tell the dom where to re-render (replace) the new Element it has created

    fn render(&self) -> Element;

    // Returns a reference to the Element in the dom so we can tell it where to put the new Element from render()
    fn element(&self) -> Option<&Element>;

    // imperative in its edit of the DOM?

    // requires that you be able to make yourself into an Element and
    // that you have a pointer to where you exist in the DOM?

    // Does websys provide an interface for replacing Elements given a pointer to it?
    // emoji of gritting teeth

    //looks promising with the Element.replace_with_node_1() method

    // We can maintain an Option<Element> that will tell us if we need to make a new Element,
    // or replace the current one?

    // Maybe it has to be Option<&Element> or similar?
    fn attach(&self, parent: Element) -> Element {}
}
