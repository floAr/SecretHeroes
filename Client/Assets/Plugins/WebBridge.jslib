mergeInto(LibraryManager.library, {

  MintTokens: function (str_json){
	var content = JSON.parse(Pointer_stringify(str_json));
	updateChart(content);
  },
  SetViewingKey: function (str_json){
	var content = JSON.parse(Pointer_stringify(str_json));
	updateChart(content);
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