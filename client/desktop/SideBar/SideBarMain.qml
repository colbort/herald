import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "qrc:/common" as Common
import "../SideBar/Header" as Header
import "../SideBar/Pane" as Pane
import "../SideBar/Pane/GroupFlowComponents" as GroupFlow
import "../SideBar/Pane/ContactFlowComponents" as ContactFlow

Page {
    id: sideBar
    property real windowFraction: width / root.width
    readonly property real maxWindowFraction: 0.66

    // maximum width, where root is ApplicationWindow
    SplitView.maximumWidth: root.width * maxWindowFraction
    SplitView.minimumWidth: 250
    SplitView.preferredWidth: 300
    padding: 0 // All Interior Elements span the entire pane

    property alias sideBarState: sideBarState

    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    ///--- SearchBar for contacts, add contact button
    header: Loader {
        id: headerLoader
        property string searchPlaceholder: ""
        property bool contactsSearch: false
        property string headerText: ""

        sourceComponent: Header.ContextBar {
            id: contextBarComponent
        }

        Common.Divider {
            anchors.top: parent.bottom
        }
    }

    //component loaded into header depending on sidebar state
    Header.HeaderComponent {
        id: headerBarComponent
    }

    //TODO: get rid of this once global search implemented
    Header.SearchComponent {
        id: searchBarComponent
    }

    GroupFlow.NewGroupComponent {
        id: newGroupComponent
    }

    ContactFlow.NewContactComponent {
        id: newContactComponent
    }

    SideBarState {
        id: sideBarState
        anchors.fill: parent
    }
}
