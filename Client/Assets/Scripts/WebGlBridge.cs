using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.Events;

public class WebGlBridge : MonoBehaviour
{
    public bool IsConnected;
    public UnityEvent Connected;
    public DrawManager DrawManager;
    public BattleMaster BattleMaster;

    [DllImport("__Internal")]
    private static extern void MintTokens();

    public void TriggerMint()
    {
        // MintTokens();
        TestRegisterMint();
    }

    [DllImport("__Internal")]
    private static extern void SendToBattle(string tokenId);

    public void TriggerBattle(Token token)
    {
        SendTestToBattle(token);
    }

    public void TriggerBattle(string id)
    {
        SendTestToBattle(id);
    }

    [DllImport("__Internal")]
    private static extern void PollData();

    public void TriggerPoll()
    {
        PolltestData();
    }


    [System.Serializable]
    public class TokenList
    {
        public Token[] tokens;
        public static TokenList CreateFromJSON(string jsonString)
        {
            return JsonUtility.FromJson<TokenList>(jsonString);
        }
    }

    private void SendTestToBattle(Token token)
    {
        Debug.Log("Token Id: " + token.id);
        GameObject.FindObjectOfType<TransitionManager>().TransitionIntoArena();
        GameObject.FindObjectOfType<BattleMaster>().OptimisticResponse(token);
        GameObject.FindObjectOfType<BattleMaster>().addOpponents(Random.Range(1, 3));
    }

    private void SendTestToBattle(string id)
    {
        Debug.Log("Alter. Token Id: " + id);
        //GameObject.FindObjectOfType<WebGlBridge>().TriggerBattle(id);
        GameObject.FindObjectOfType<TransitionManager>().TransitionIntoArena();
        //GameObject.FindObjectOfType<BattleMaster>().OptimisticResponse(token);
    }

    private void PolltestData()
    {
        TestReportTokens();
    }
    public void RegisterMint(string encodedTokens)
    {
        Debug.Log("Encoded for draw" + encodedTokens);
        var json = "{ \"tokens\":" + encodedTokens + "}";
        var tokens = TokenList.CreateFromJSON(json);
        GameObject.FindObjectOfType<DrawManager>().PrepareSlots(tokens.tokens);
        GameObject.FindObjectOfType<TransitionManager>().TransitionIntoMarket();

        ReportTokens(encodedTokens);
    }

    public void RegisterMintByOne(string encodedTokens)
    {
        Debug.Log("Encoded for draw" + encodedTokens);
        var json = encodedTokens;
        var token = Token.CreateFromJSON(json);
        GameObject.FindObjectOfType<DrawManager>().PrepareSlot(token);
        GameObject.FindObjectOfType<TransitionManager>().TransitionIntoMarket();
    }

    public void ReportTokens(string encodedTokens)
    {
        Debug.Log("Encoded for rooster" + encodedTokens);
        var json = "{ \"tokens\":" + encodedTokens + "}";
        var tokens = TokenList.CreateFromJSON(json);
        GameObject.FindObjectOfType<Rooster>().UpdateTokens(tokens.tokens);
        GameObject.FindObjectOfType<SelectionRooster>().Refresh();
    }

    public void ReportToken(string encodedTokens)
    {
        Debug.Log("Encoded for rooster" + encodedTokens);
        var json = encodedTokens;
        var token = Token.CreateFromJSON(json);
        GameObject.FindObjectOfType<Rooster>().UpdateToken(token);
        GameObject.FindObjectOfType<SelectionRooster>().Refresh();
    }

    public void ReportBattleStatus(string encodedBattle)
    {
        Debug.Log("Encoded for battleState " + encodedBattle);
        var battle = BattleState.CreateFromJSON(encodedBattle);
        BattleMaster.AddBattleState(battle);
    }

    public void Connect()
    {
        StartCoroutine(EnableAfterX(2));
    }

    private IEnumerator EnableAfterX(int seconds)
    {
        yield return new WaitForSeconds(seconds);
        IsConnected = true;
        if (Connected != null)
            Connected.Invoke();
    }

    [ContextMenu("Test RegisterMint")]
    public void TestRegisterMint()
    {
        RegisterMint(createRandomTokens());
    }

    [ContextMenu("Test RegisterOneMint")]
    public void TestRegisterOneMint()
    {
        RegisterMintByOne(createRandomToken());
    }

    [ContextMenu("Test ReportTokens")]
    public void TestReportTokens()
    {
        ReportTokens(createRandomTokens());
    }

    [ContextMenu("Test ReportOneToken")]
    public void TestReportToken()
    {
        ReportToken(createRandomToken());
    }

    [ContextMenu("Test Connect")]
    public void TestConnect()
    {
        Connect();
    }
    [ContextMenu("Test ReportBattleState")]
    public void TestReportBattleState()
    {
        ReportBattleStatus("{\"heroes_waiting\": 0, \"your_hero\": {\"token_id\": \"test\",\"name\": \"test\",\"skills\": [17,2,14,7]}}");
    }

    [ContextMenu("Test ReportBattleState222")]
    public void TestReportBattleStateZEor()
    {
        ReportBattleStatus("{\"heroes_waiting\": 1, \"your_hero\": null}");
    }
    [ContextMenu("Test ReportBattleState2222222")]
    public void TestReportBattleStateZEor2()
    {
        ReportBattleStatus("{\"heroes_waiting\": 2, \"your_hero\": null}");
    }

    private string createRandomTokens()
    {
        string json = "";
        json = "[" +
            createRandomToken() + "," +
            createRandomToken() + "," +
            createRandomToken() + "]";
        return json;
    }

    private string createRandomToken()
    {
        string json = "";
        Token tempToken = new Token();
        tempToken = Token.Random();
        tempToken.name += tempToken.id;
        json = JsonUtility.ToJson(tempToken);
        return json;
    }

}