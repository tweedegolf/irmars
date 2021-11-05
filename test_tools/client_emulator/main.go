package main

import (
	"bufio"
	"fmt"
	"os"
	"time"

	irma "github.com/privacybydesign/irmago"
	"github.com/privacybydesign/irmago/irmaclient"
)

type ClientHandler struct {
}

func (_ *ClientHandler) EnrollmentFailure(manager irma.SchemeManagerIdentifier, err error) {
	panic("Unexpected call to EnrollmentFailure")
}

func (_ *ClientHandler) EnrollmentSuccess(manager irma.SchemeManagerIdentifier) {
	panic("Unexpected call to EnrollmentSuccess")
}

func (_ *ClientHandler) ChangePinFailure(manager irma.SchemeManagerIdentifier, err error) {
	panic("Unexpected call to ChangePinFailure")
}

func (_ *ClientHandler) ChangePinSuccess(manager irma.SchemeManagerIdentifier) {
	panic("Unexpected call to ChangePinSuccess")
}

func (_ *ClientHandler) ChangePinIncorrect(manager irma.SchemeManagerIdentifier, attempts int) {
	panic("Unexpected call to ChangePinIncorrect")
}

func (_ *ClientHandler) ChangePinBlocked(manager irma.SchemeManagerIdentifier, timeout int) {
	panic("Unexpected call to ChangePinBlocked")
}

func (_ *ClientHandler) UpdateConfiguration(new *irma.IrmaIdentifierSet) {
	panic("Unexpected call to UpdateConfiguration")
}

func (_ *ClientHandler) UpdateAttributes() {
	fmt.Println("Received new credential")
}

func (_ *ClientHandler) Revoked(cred *irma.CredentialIdentifier) {
	panic("Unexpected call to Revoked")
}

func (_ *ClientHandler) ReportError(err error) {
	panic("Unexpected call to ReportError")
}

type SessionHandler struct {
	completion chan<- struct{}
	reader     *bufio.Reader
}

func (_ *SessionHandler) StatusUpdate(action irma.Action, status irma.ClientStatus) {
	fmt.Println(status)
}

func (_ *SessionHandler) ClientReturnURLSet(clientReturnURL string) {
	panic("Unexpected call to ClientReturnURLSet")
}

func (_ *SessionHandler) PairingRequired(pairingCode string) {
	panic("Unexpected call to PairingRequired")
}

func (s *SessionHandler) Success(result string) {
	s.completion <- struct{}{}
}

func (s *SessionHandler) Cancelled() {
	time.Sleep(1 * time.Second)
	s.completion <- struct{}{}
}

func (_ *SessionHandler) Failure(err *irma.SessionError) {
	panic("Unexpected call to Failure")
}

func (_ *SessionHandler) KeyshareBlocked(manager irma.SchemeManagerIdentifier, duration int) {
	panic("Unexpected call to KeyshareBlocked")
}

func (_ *SessionHandler) KeyshareEnrollmentIncomplete(manager irma.SchemeManagerIdentifier) {
	panic("Unexpected call to KeyshareEnrollmentIncomplete")
}

func (_ *SessionHandler) KeyshareEnrollmentMissing(manager irma.SchemeManagerIdentifier) {
	panic("Unexpected call to KeyshareEnrollmentMissing")
}

func (_ *SessionHandler) KeyshareEnrollmentDeleted(manager irma.SchemeManagerIdentifier) {
	panic("Unexpected calll to KeyshareEnrollmentDeleted")
}

func makeFirstDisclosureChoice(candidates [][]irmaclient.DisclosureCandidates) *irma.DisclosureChoice {
	attributes := [][]*irma.AttributeIdentifier{}
	for i := range candidates {
		choice, err := candidates[i][0].Choose()
		if err != nil {
			panic(err)
		}
		attributes = append(attributes, choice)
	}
	return &irma.DisclosureChoice{
		Attributes: attributes,
	}
}

func (s *SessionHandler) shouldCancel() bool {
	command, err := s.reader.ReadString('\n')
	if err != nil {
		panic(err)
	}
	return command == "cancel\n"
}

func (s *SessionHandler) RequestIssuancePermission(request *irma.IssuanceRequest,
	satisfiable bool,
	candidates [][]irmaclient.DisclosureCandidates,
	requestorInfo *irma.RequestorInfo,
	callback irmaclient.PermissionHandler) {
	if s.shouldCancel() {
		callback(false, nil)
	} else {
		callback(true, makeFirstDisclosureChoice(candidates))
	}
}

func (s *SessionHandler) RequestVerificationPermission(request *irma.DisclosureRequest,
	satisfiable bool,
	candidates [][]irmaclient.DisclosureCandidates,
	requestorInfo *irma.RequestorInfo,
	callback irmaclient.PermissionHandler) {
	if s.shouldCancel() {
		callback(false, nil)
	} else {
		callback(true, makeFirstDisclosureChoice(candidates))
	}
}

func (s *SessionHandler) RequestSignaturePermission(request *irma.SignatureRequest,
	satisfiable bool,
	candidates [][]irmaclient.DisclosureCandidates,
	requestorInfo *irma.RequestorInfo,
	callback irmaclient.PermissionHandler) {
	if s.shouldCancel() {
		callback(false, nil)
	} else {
		callback(true, makeFirstDisclosureChoice(candidates))
	}
}

func (_ *SessionHandler) RequestSchemeManagerPermission(manager *irma.SchemeManager,
	callback func(proceed bool)) {
	panic("Unexpected call to RequestSchemeManagerPermission")
}

func (_ *SessionHandler) RequestPin(remainingAttempts int, callback irmaclient.PinHandler) {
	panic("Unexpected call to RequestPin")
}

func main() {
	client, err := irmaclient.New(
		"temp_testing/client",
		"temp_testing/irma_configuration",
		&ClientHandler{},
	)

	client.SetPreferences(irmaclient.Preferences{DeveloperMode: true})

	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(os.Stdin)
	sessionptr, err := reader.ReadString('\n')
	if err != nil {
		panic(err)
	}

	c := make(chan struct{})

	client.NewSession(sessionptr, &SessionHandler{completion: c, reader: reader})

	<-c

	client.Close()
}
