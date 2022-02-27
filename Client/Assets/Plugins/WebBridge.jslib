mergeInto(LibraryManager.library, {

  MintTokens: function (){
	window.mint()
  },
  
  SendToBattle: function (tokenId){
	var token = Pointer_stringify(tokenId);
	window.sendToBattle(token)
  },


 UpgradeHeroes: function(upgradeId, burnOne, burnTwo){
    var upgrade = Pointer_stringify(upgradeId);
    var b1 = Pointer_stringify(burnOne);
    var b2 = Pointer_stringify(burnTwo);
    window.ugradeHeroes(upgrade, b1, b2)
 };

  PollData: function (){
	window.poll()
  }
});