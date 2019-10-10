# Outline

Concept is to create a basic UI DSL (stubbed for simplicity here), with which we can create a user interface.  The DSL should provide support for several basic UI components, and should be renderable via a render function.

To do this, we should create a render trait, which should be implemented by any components we wish to support.

Alongside the render trait, we should implement a render function, which takes a collection of items which implement the render trait (a collection of trait objects).  The key here is that we should not depend on specific types, only that the collection implements the render trait itself.

The render function should output a simple representation of what the UI should look like, for example, a console output of descriptive text, or a JSON rendition of the document etc.

## Suggested types for the UI DSL
- Divider
- Label
- TextBox
- List
