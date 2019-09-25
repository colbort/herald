///*
// * a memory tuple class
// **/
//export class ChatViewMemory {
//  scrollMemory: number;
//  textMemory: string;
//
//  constructor() {
//    this.scrollMemory = Number(1.0);
//    this.textMemory = String("");
//  }
//}
//
///*
// * global object to keep track of  chat area memory
// **/
//export const textAreaMemory = {
//  currentConversationid: "",
//  invalid: new ChatViewMemory()
//};
//
///*
// * Gets chatview memory with the current conversationID
// * on falsey conversation id returns
// **/
//export function getTextAreaMemory(
//  conversationID: number
//): ChatViewMemory | undefined {
//  if (!!!conversationID) {
//    return textAreaMemory.invalid;
//  }
//}
export function enterKeyHandler(event, target, networkHandle, messageModel) {
    console.log("called");
    if (event.modifiers & Qt.ShiftModifier) {
        console.log("shift mod");
        target.text = target.text + "\n";
        target.cursorPosition = target.text.length;
        return;
    }
    if (target.text.trim().length <= 0) {
        console.log("early return, no text");
        return;
    }
    // clear before positional reset
    const text = target.text;
    target.clear();
    console.log("cleared");
    const messageId = messageModel.insertMessage(text);
    console.log("inserted");
    networkHandle.sendMessage(text, messageModel.conversationId, messageId);
    console.log("sent");
}
