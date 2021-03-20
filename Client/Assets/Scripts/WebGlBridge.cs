using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class WebGlBridge : MonoBehaviour
{
    public DrawManager DrawManager;

    [DllImport("__Internal")]
    private static extern void MintTokens(string str);

    public void TriggerMint()
    {
        //Debug.Log(payload);
        MintTokens("");
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
        Debug.Log("Encoded " + encodedTokens);
        var json = "{ \"tokens\":" + encodedTokens + "}";
        var tokens = TokenList.CreateFromJSON(json);
        GameObject.FindObjectOfType<DrawManager>().PrepareSlots(tokens.tokens);
        GameObject.FindObjectOfType<TransitionManager>().TransitionIntoMarket();
    }

    [ContextMenu("Test RegisterMint")]
    public void TestRegisterMint() {
        RegisterMint("[{\"name\":\"unity3d\",\"weapons\":1,\"engineering\":2,\"biotech\":3,\"psychics\":4},{\"name\":\"webgl\",\"weapons\":5,\"engineering\":6,\"biotech\":7,\"psychics\":8},{\"name\":\"suck it\",\"weapons\":9,\"engineering\":10,\"biotech\":11,\"psychics\":12}]");
    }
}
