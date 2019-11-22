import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "../SideBar/Header" as Header
import "../SideBar/Pane" as Pane
import "../SideBar/Pane/GroupFlowComponents" as GroupFlow


Page {
    id: sideBarStateLoader
    padding: 0

    background: Rectangle {
        color: CmnCfg.palette.paneColor
    }

    Component {
        id: contactslvComponent
        Pane.ContactView {
            id: contactsListView
            anchors.fill: parent
            model: contactsModel
        }
    }

    Pane.SideBarPane {
        id: sideBarPane
    }

    states: [
        State {
            name: "newContactState"
            PropertyChanges {
                target: sideBarPane.sideBarBodyLoader
                sourceComponent: newContactComponent
            }
            PropertyChanges {
                target: headerLoader
                sourceComponent: headerBarComponent
                searchPlaceholder: "Search your conversations"
                headerText: "Add contact"
            }
        },

        State {
            name: "newGroupState"
            PropertyChanges {
                target: sideBarPane.sideBarBodyLoader
                sourceComponent: newGroupComponent
            }

            PropertyChanges {
                target: headerLoader
                sourceComponent: headerBarComponent
                headerText: "New group"
                contactsSearch: true
            }
        },

        State {
            name: "globalSearch"
            PropertyChanges {
                target: headerLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Search your conversations"
            }

            //load model into view
            PropertyChanges {
                target: sideBarPane.messageSearchLoader
                searchModel: herald.messageSearch
            }
        },

        //TODO: following state should be reworked to match new design
        State {
            name: "newConversationState"
            PropertyChanges {
                target: sideBarBodyLoader
                sourceComponent: contactslvComponent
            }

            PropertyChanges {
                target: headerLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Enter contact name"
                contactsSearch: true
            }
        }
    ]
}
