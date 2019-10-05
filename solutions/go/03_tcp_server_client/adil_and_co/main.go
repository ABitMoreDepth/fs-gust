package main

import (
	"flag"
	"fmt"
	"net"
)

func main() {
	port := flag.String("Port", "1234", "Port to connect to.")
	name := flag.String("Name", "Elvis", "Your name")
	connection, err := net.Dial("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		panic(fmt.Sprintf("Could not connect to: %s", port))
	}
	defer connection.Close()

	var data []byte
	greeting := fmt.Sprintf("Hello my name is %s\x00", name)
	_, err = connection.Write([]byte(greeting))
	if err != nil {
		fmt.Println(err)
	}

	_, err = connection.Read(data)
	if err != nil {
		fmt.Println(err)
	}

	_, err = connection.Read(data)
	if err != nil {
		fmt.Println(err)
	}

	fmt.Println(data)
}
