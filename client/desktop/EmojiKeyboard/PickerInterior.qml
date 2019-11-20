import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Item {
    property color lowlight: "light gray"
    // header and search bar
    Item {
        id: header
        height: 30 //enough for search bar of default size w/ margins
        anchors.top: parent.top
        anchors.topMargin: CmnCfg.smallMargin
        anchors.right: parent.right
        anchors.left: parent.left

        // search bar and exit button
        Rectangle {
            id: taBox
            anchors {
                left: parent.left
                right: menu.left
                margins: CmnCfg.smallMargin
            }
            color: "#33000000" // transparent
            border.color: CmnCfg.palette.sideBarHighlightColor
            height: 24
            Row {
                anchors.fill: parent
                spacing: 0
                Button {
                    padding: 0
                    background: Item {}
                    icon.source: "qrc:/search-icon.svg"
                    icon.color: CmnCfg.palette.sideBarHighlightColor
                    icon.height: 17
                    icon.width: 17
                    anchors.verticalCenter: parent.verticalCenter
                }

                ScrollView {
                    width: 185
                    TextArea {
                        id: searchTextArea
                        padding: 0
                        color: CmnCfg.palette.sideBarHighlightColor
                        placeholderText: "Search emoji"
                        Keys.onReturnPressed: event.accepted = true
                        anchors.fill: parent
                    }
                    anchors.verticalCenter: parent.verticalCenter
                }

                Button {
                    id: exitButton
                    padding: 0
                    background: Item {}
                    icon.source: "qrc:/x-icon.svg"
                    icon.color: CmnCfg.palette.sideBarHighlightColor
                    onClicked: emoKeysPopup.active = false
                    icon.height: 17
                    icon.width: 17
                    anchors.verticalCenter: parent.verticalCenter
                }
            }
        }

        // skin swatch selector
        ComboBox {
            id: menu
            anchors.right: parent.right
            anchors.margins: CmnCfg.margin
            anchors.verticalCenter: taBox.verticalCenter
            height: 20
            width: 20
            currentIndex: CmnCfg.skinSwatchIndex
            model: ["#f4be40", "#f9dcbe", "#dfbb97", "#c18f6b", "#9a6440", "#59453a"]
            indicator: Item {}

            delegate: ItemDelegate {
                height: menu.height
                Rectangle {
                    anchors.fill: parent
                    color: menu.model[index]
                }
            }

            onCurrentIndexChanged: {
                CmnCfg.skinSwatchIndex = currentIndex
            }

            contentItem: Rectangle {
                anchors.fill: parent
                border.color: CmnCfg.palette.secondaryTextColor
                color: menu.model[menu.currentIndex]
            }
        }
    }

    // actual interior
    Item {
        id: listView
        width: parent.width

        anchors {
            top: header.bottom
            bottom: footer.top
        }

        Flickable {
            id: emojiList
            anchors.fill: parent
            boundsBehavior: Flickable.StopAtBounds
            clip: true
            ScrollBar.vertical: ScrollBar {}
            contentHeight: innerCol.height
            Column {
                id: innerCol
                Repeater {
                    id: innerRepeater
                    model: searchTextArea.text.length ? [] : CmnCfg.emojiModel
                    Column {
                        padding: CmnCfg.smallMargin
                        Label {
                            text: modelData.sectionName
                            color: CmnCfg.palette.sideBarHighlightColor
                            font.bold: true
                            font.family: CmnCfg.chatFont.name
                            bottomPadding: CmnCfg.smallMargin
                        }

                        Loader {
                            asynchronous: index !== 0
                            sourceComponent: Grid {
                                id: emojiGrid
                                columns: 10
                                spacing: 7
                                width: listView.width
                                Repeater {
                                    id: self
                                    model: modelData.List
                                    EmojiButton {
                                        baseEmoji: self.model[index][0]
                                        takesModifier: self.model[index].length === 3
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // footer and anchor links
    Item {
        id: footer

        anchors.bottom: parent.bottom
        width: parent.width
        height: 30

        Rectangle {
            id: hr
            width: parent.width
            height: 1
            color: CmnCfg.palette.secondaryTextColor
        }

        RowLayout {
            anchors.fill: parent
            anchors.margins: 8
            spacing: CmnCfg.smallMargin

            AnchorButton {
                anchorIndex: 0
                imageSource: "qrc:/emoji-categories/gestural.svg"
            }
            AnchorButton {
                anchorIndex: 1
                imageSource: "qrc:/emoji-categories/nature.svg"
            }
            AnchorButton {
                anchorIndex: 2
                imageSource: "qrc:/emoji-categories/food.svg"
            }
            AnchorButton {
                anchorIndex: 3
                imageSource: "qrc:/emoji-categories/transport.svg"
            }
            AnchorButton {
                anchorIndex: 4
                imageSource: "qrc:/emoji-categories/sports.svg"
            }
            AnchorButton {
                anchorIndex: 5
                imageSource: "qrc:/emoji-categories/items.svg"
            }
            AnchorButton {
                anchorIndex: 6
                imageSource: "qrc:/emoji-categories/symbols.svg"
            }
            AnchorButton {
                anchorIndex: 7
                imageSource: "qrc:/emoji-categories/flags.svg"
            }
        }
    }
}
