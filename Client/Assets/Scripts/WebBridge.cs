using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class WebBridge : MonoBehaviour
{
    [DllImport("__Internal")]
    private static extern void MintTokens(string str);

    public void Mint()
    {
        //Debug.Log(payload);
        MintTokens("");
    }

}
