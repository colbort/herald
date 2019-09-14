import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import QtQml 2.13

Item {
    id: appRoot

    property var gsConversationId

    anchors.fill: parent.fill
    Layout.fillWidth: true
    Layout.fillHeight: true

    NetworkHandle {
        id: networkHandle

    }

    Messages {
        id: messageModel
    }

    Contacts {
        id: contactsModel
    }

    Popups.ConfigPopup {
        id: preferencesPopup
    }

    Config {
        id: config
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
            implicitWidth: 3
            color: QmlCfg.palette.secondaryColor
        }
    }
}
