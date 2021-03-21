mergeInto(LibraryManager.library, {

  MintTokens: function (str_json){
	var content = JSON.parse(Pointer_stringify(str_json));
	updateChart(content);
  },
  
  SendToBattle: function (tokenId){
	var token = Pointer_stringify(tokenId);
	updateChart(content);
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