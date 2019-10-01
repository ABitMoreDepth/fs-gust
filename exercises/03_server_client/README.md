# Outline
The base concept is to create a simple server and client pair of programs, speaking basic TCP to one another.

## Client Spec
The client must attempt to connect to the server on port 1234, and should transmit a UTF-8 string within a TCP packet, with the phrase `Hello, my name is {}`, where `{}` should be the client's single-word name of choice.

The client must expect a response packet from the server matching `Hello {}`, where `{}` is equal to the clients name.

## Server Spec
The Server must accept connections to port 1234.  Connections should contain the above formatted phrase encoded with UTF-8.

The server must respond to the TCP connection with a UTF-8 formatted string comprising: `Hello, {}`, where `{}` is equal to the client name.
