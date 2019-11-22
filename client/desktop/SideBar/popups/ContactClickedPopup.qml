import QtQuick 2.13
import LibHerald 1.0
import QtQuick 2.0
import QtQuick.Controls 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "qrc:/imports/utils.mjs" as Utils
import "./js/ContactClickedPopup.mjs" as JS
import "../popups/js/" as Popups

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC

/// --- displays a list of contacts

//TODO: when UI design is set make this work
Item {
    property alias optionsMenu: optionsMenu
    // GS: These should be bound to global scope. handled ENTIRELY
    // by the Contacts model functions.
    FileDialog {
        id: pfpDialog
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        folder: shortcuts.desktop
        onSelectionAccepted: JS.changeProfilePicture(index, contactsModel,
                                                     fileUrl, this)
    }

    Menu {
        id: optionsMenu

        MenuSeparator {
        }

        MenuItem {
            text: 'Rename Contact'
            // Note: remove , because this is a testing feature.
            // instead networking needs to know...
            onTriggered: renameContactDialogue.open()
        }

        MenuSeparator {
        }

        MenuItem {
            // Note: remove , because this is a testing feature
            // instead networking needs to know...
            text: 'Choose Avatar'
            onTriggered: pfpDialog.open()
        }

        MenuItem {
            // Note: remove , because this is a testing feature.
            // instead networking needs to know...
            text: 'Clear Avatar'
            onTriggered: {
                contactAvatar.pfpUrl = ""
                contactsModel.setProfilePicture(index, "")
            }
        }

        MenuItem {
            text: 'Choose Color'

            onTriggered: {
                // TODO pass an argument
                // Paul 0: see Paul:0
                gsSelectedIndex = index
                avatarColorPicker.show()
            }
        }
    }

    Popup {
        id: renameContactDialogue
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        width: CmnCfg.popupWidth
        height: CmnCfg.popupHeight

        TextArea {
            id: entryField
            focus: true
            placeholderText: qsTr("Enter new name")
            Keys.onReturnPressed: JS.renameContact(index, entryField,
                                                   renameContactDialogue,
                                                   contactsModel)
            anchors.fill: parent
            wrapMode: TextEdit.WrapAnywhere
        }

        Button {
            id: submissionButton
            text: "Submit"
            anchors {
                bottom: parent.bottom
                right: parent.right
            }
            onClicked: JS.renameContact(index, entryField,
                                        renameContactDialogue, contactsModel)
        }
    }
}
