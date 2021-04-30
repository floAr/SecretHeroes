mergeInto(LibraryManager.library, {

  MintTokens: function (){
	window.mint()
  },
  
  SendToBattle: function (tokenId){
	var token = Pointer_stringify(tokenId);
	window.sendToBattle(token)
  },

  PollData: function (){
	window.poll()
  }
});