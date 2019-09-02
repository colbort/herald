import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import "common/utils.js" as Utils
ApplicationWindow {
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    id: root
    minimumWidth: 250
    minimumHeight: 300

    NetworkHandle {
        id: networkHandle
        onNewMessageChanged: {
            print("message received.")
        }

        onConnectionUpChanged: {
            print("network up status is : ", connectionPending)
        }

        onConnectionPendingChanged: {
            print("network pending status is : ", connectionPending)
        }
    }

    Messages {
        property string currentId : Utils.try_index_or(
                                             sideBar.contactUi.currentIndex,
                                             sideBar.contactData.contact_id,
                                             sideBar.contactData.rowCount(),"")
        id: messageModel
        conversationId: currentId
    }

    Popups.ConfigPopup {
        id: firstTimePopup
    }

    /// global configurations item
    /// ToDo : implement login page logic here instead
    Config {
        id: config
        Component.onCompleted: {
            if (!config.exists()) {
                firstTimePopup.open()
                print("placeholder for a popup which forces first time config.")
            }
        }
    }

    /// Todo : make the split handle less intrusive. probably just a line
    SplitView {
        id: rootSplitView
        anchors.fill: parent
        Layout.fillWidth: true
        Layout.fillHeight: true
        orientation: Qt.Horizontal

        /// Contacts view for the desktop client, in DesktopContacts.qml
        /// includes the config and contacts toolbars
        SideBar {
            id: sideBar
        }

        ChatView {
            id: chatView
        }

        handle: Rectangle {
            implicitWidth: 4
            implicitHeight: 4
            color: if (SplitHandle.pressed) {
                       Qt.darker(QmlCfg.palette.secondaryColor, 1.1)
                   } else {
                       QmlCfg.palette.secondaryColor
                   }
        }
    }
}
