import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/utils.mjs" as Utils
import "./ContactView.mjs" as JS
import "popups" as Popups

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC

/// --- displays a list of contacts
ListView {
    id: contactList

    clip: true
    currentIndex: -1
    boundsBehavior: Flickable.StopAtBounds

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: contactItem

        //GS : rexporting the contact avatar to global state is a backwards ref!
        property Item contactAvatar: contactAvatar

        height: JS.contactItemHeight(visible)
        width: parent.width
        visible: matched

        /// NPB : THis ought to be a mouse area with a hovered handler
        Rectangle {
            id: bgBox
            readonly property color focusColor: QmlCfg.palette.tertiaryColor
            readonly property color hoverColor: QmlCfg.palette.secondaryColor
            readonly property color defaultColor: QmlCfg.palette.mainColor

            /// FC: ANOTHER BORDER!
            Rectangle {
                anchors.verticalCenter: parent.bottom
                color: QmlCfg.palette.secondaryColor
                width: parent.width
                height: 1.5
            }

            anchors.fill: parent

            /// Note: can we use the highlight property here
            states: [
                State {
                    name: "hovering"
                    PropertyChanges {
                        target: bgBox
                        color: hoverColor
                    }
                },
                State {
                    name: "focused"
                    PropertyChanges {
                        target: bgBox
                        color: focusColor
                    }
                }
            ]

            MouseArea {
                hoverEnabled: true
                z: 10
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onEntered: parent.state = "hovering"
                onExited: parent.state = ""

                onClicked: JS.contactClickHandler(mouse, contactList, index,
                                                  contactId, contactItem,
                                                  popupManager.optionsMenu,
                                                  messageModel, chatView)

                onReleased: parent.state = Utils.safeSwitch(containsMouse,
                                                            "hovering", "")
            }

            ///NPB : see the QT labs menu import. [https://doc.qt.io/qt-5/qml-qt-labs-platform-menu.html]
            Popups.ContactClickedPopup {
                id: popupManager
            }
            color: Utils.safeSwitch(contactItem.focus, focusColor, defaultColor)
        }

        /// NPB: Make ALL calls to model proerties use the Explicit row syntax.
        /// NPB: unwrapOr should use a subset of falsey values to coerce to false, maybe make a tryGetOr(getter *fn , index, failValue)
        /// NB: Where is  index coming from?? (Positioner, but this is so implicit that we hate it)
        Common.Avatar {
            size: 50
            id: contactAvatar
            /// NPB: use camel case in libherald please
            displayName: Utils.unwrapOr(name, contactId)
            colorHash: color
            /// NPB: use camel case in libherald please
            pfpUrl: Utils.unwrapOr(profilePicture, null)
        }
    }
}
