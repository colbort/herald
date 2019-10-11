import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import Qt.labs.settings 1.0
import "SideBar/popups" as Popups
import "errors" as ErrorUtils
import QtQml 2.13

ApplicationWindow {
    id: root
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    minimumWidth: 500
    minimumHeight: 300

    TopMenuBar {}

    Errors {
        id: errorQueue

        onTryPollChanged: {
            let errMsg = errorQueue.nextError()
            if (errMsg !== "") {
                errPopup.errorMsg = errMsg
                errPopup.open()
            }
        }

        property var errPopup: ErrorUtils.ErrorDialog {}
    }

    // Paul 7: move these utils and state to a ```globals``` qml module.
    // This provides a few purely functional helper methods
    HeraldUtils {
        id: heraldUtils
    }

    HeraldState {
        id: heraldState
    }

    NetworkHandle {
        id: networkHandle
    }

    Loader {
        id: appLoader
        active: heraldState.configInit
        anchors.fill: parent
        sourceComponent: App {}
    }

    Loader {
        anchors.fill: parent
        id: registrationLoader
        active: !heraldState.configInit
        sourceComponent: RegistrationPage {}
    }
}
