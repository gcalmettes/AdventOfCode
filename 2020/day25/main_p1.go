package main

import "fmt"

func main() {

	cardPublicKey := 9093927
	doorPublicKey := 11001876

	subject := 7

	var cardLoopSize, doorLoopSize int

	in := 1
	i := 1
	for {
		in = loop(in, subject)
		if in == cardPublicKey {
			cardLoopSize = i
		}
		if in == doorPublicKey {
			doorLoopSize = i
		}

		if (cardLoopSize != 0) && (doorLoopSize != 0) {
			break
		}
		i++
	}

	cardEncryptionKey := 1
	doorEncryptionKey := 1

	for i := 0; i < cardLoopSize; i++ {
		cardEncryptionKey = loop(cardEncryptionKey, doorPublicKey)
	}

	for i := 0; i < doorLoopSize; i++ {
		doorEncryptionKey = loop(doorEncryptionKey, cardPublicKey)
	}

	if cardEncryptionKey == doorEncryptionKey {
		fmt.Println(cardEncryptionKey)
	} else {
		fmt.Println("Did not crack the encryption!")
	}

}

func loop(in, subject int) int {
	in *= subject
	in = in % 20201227
	return in
}
