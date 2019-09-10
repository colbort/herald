import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import QtQml 2.13

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
ApplicationWindow {
    id: root
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    minimumWidth: 250
    minimumHeight: 300

    TopMenuBar {
    }

    NetworkHandle {
        id: networkHandle
    }

    Messages {
        id: messageModel
    }

    Contacts {
        id: contactsModel
    }

    // NPB : always instantiated, more like a state, or a page than a popup
    Popups.ConfigPopup {
        id: preferencesPopup
    }

    Config {
        id: config
        Component.onCompleted: {
            if (!config.exists()) {
                preferencesPopup.open()
                print("placeholder for a popup which forces first time config.")
            }
        }
    }

    SplitView {
        id: rootSplitView
        anchors.fill: parent
        Layout.fillWidth: true
        Layout.fillHeight: true
        orientation: Qt.Horizontal

        SideBar {
            id: sideBar
        }

        ChatView {
            id: chatView
        }

        handle: Rectangle {
            implicitWidth: 2
            color: QmlCfg.palette.secondaryColor
        }
    }
}
