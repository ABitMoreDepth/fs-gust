package main

import (
	"fmt"
)

type renderable interface {
	render() string
}

type listItem struct {
	content string
}

type unorderedList struct {
	content []listItem
}

type div struct {
	content []renderable
}

func (l listItem) render() string {
	return fmt.Sprintf("<li>%s</li>", l.content)
}

func (u unorderedList) render() string {
	var ul string = "<ul>"
	for n := 0; n < len(u.content); n++ {
		ul += u.content[n].render()
	}
	ul += "</ul>"
	return ul
}

func (d div) render() string {
	var div string = "<div>"
	for n := 0; n < len(d.content); n++ {
		div += d.content[n].render()
	}
	div += "</div>"
	return div
}

func makePage(r []renderable) {
	for n := 0; n < len(r); n++ {
		fmt.Println(r[n].render())
	}
}

func main() {
	listItem1 := listItem{content: "I'm a list"}
	listItem2 := listItem{content: "I'm another list"}

	listItemArray := make([]listItem, 0)
	listItemArray = append(listItemArray, listItem1)
	listItemArray = append(listItemArray, listItem2)

	unorderedListContainer := make([]renderable, 0)
	unorderedListContainer = append(unorderedListContainer, unorderedList{content: listItemArray})

	divContainer := make([]renderable, 0)
	divContainer = append(divContainer, div{content: unorderedListContainer})

	makePage(divContainer)
}
