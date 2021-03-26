mergeInto(LibraryManager.library, {

  MintTokens: function (){
	window.scrtMint()
  },
  
  SendToBattle: function (tokenId){
	var token = Pointer_stringify(tokenId);
	window.scrtSentToBattle(token)
  },

  PollData: function (){
	window.scrtPoll()
  },

   GetPrivateMetadata: function (str_json){
	var content = JSON.parse(Pointer_stringify(str_json));
	updateChart(content);
  },

   GetTokens: function (str_json){
	var content = JSON.parse(Pointer_stringify(str_json));
	updateChart(content);
  }
});