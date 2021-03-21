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
    private static extern void MintTokens(string str);

    public void TriggerMint()
    {
        //Debug.Log(payload);
        MintTokens("");
    }

    [DllImport("__Internal")]
    private static extern void PollData();

    public void TriggerPoll()
    {
        //Debug.Log(payload);

        PollData();
    }


    [DllImport("__Internal")]
    private static extern void SendToBattle(string tokenId);

    public void SendToBattle()
    {
        //Debug.Log(payload);
        SendToBattle("");
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

    public void RegisterMint(string encodedTokens)
    {
        Debug.Log("Encoded for draw" + encodedTokens);
        var json = "{ \"tokens\":" + encodedTokens + "}";
        var tokens = TokenList.CreateFromJSON(json);
        GameObject.FindObjectOfType<DrawManager>().PrepareSlots(tokens.tokens);
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

    public void ReportBattleStatus(string encodedBattle)
    {
        Debug.Log("Encoded for battleState " + encodedBattle);
        var battle = BattleState.CreateFromJSON(encodedBattle);
        BattleMaster.AddBattleState(battle);
    }

    public void Connect()
    {
        IsConnected = true;
        if (Connected != null)
            Connected.Invoke();
    }

    [ContextMenu("Test RegisterMint")]
    public void TestRegisterMint()
    {
        RegisterMint("[{\"name\":\"unity3d\",\"weapons\":1,\"engineering\":2,\"biotech\":3,\"psychics\":4},{\"name\":\"webgl\",\"weapons\":5,\"engineering\":6,\"biotech\":7,\"psychics\":8},{\"name\":\"suck it\",\"weapons\":9,\"engineering\":10,\"biotech\":11,\"psychics\":12}]");
    }

    [ContextMenu("Test ReportToken")]
    public void TestReportToken()
    {
        ReportTokens("[{\"name\":\"unity3d\",\"weapons\":1,\"engineering\":2,\"biotech\":3,\"psychics\":4},{\"name\":\"webgl\",\"weapons\":5,\"engineering\":6,\"biotech\":7,\"psychics\":8},{\"name\":\"suck it\",\"weapons\":9,\"engineering\":10,\"biotech\":11,\"psychics\":12}]");
    }

    [ContextMenu("Test Connect")]
    public void TestConnect()
    {
        Connect();
    }
    [ContextMenu("Test ReportBattleState")]
    public void TestReportBattleState()
    {
        ReportBattleStatus("{\"heroes_waiting\": 1, \"your_hero\": {\"token_id\": \"test\",\"name\": \"test\",\"skills\": [17,2,14,7]}}");
    }

    [ContextMenu("Test ReportBattleState222")]
    public void TestReportBattleStateZEor()
    {
        ReportBattleStatus("{\"heroes_waiting\": 1, \"your_hero\": null}");
    }
}

