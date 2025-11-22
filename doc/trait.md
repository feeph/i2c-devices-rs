# Implementing the I²C Bus Device (IBD) trait

In order to provide a device-agnostic library a trait is used to define a
generic interface for I²C bus devices. The library does not care which
exact calls need to be made to the hardware, it just needs a generic way to
make them.

Please refer to the provided [examples](../examples/) how to implement the trait.
