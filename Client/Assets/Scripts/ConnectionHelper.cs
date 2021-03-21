using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;

public class ConnectionHelper : MonoBehaviour
{
    public Canvas ConnectionCanvas;
    public CameraController Controller;
    public TMP_Text ConnectingString;

    private string[] connectionString = new string[]
    {
        "Connecting.",
        "Connecting..",
        "Connecting...",
        "Connecting....",
    };

    private void Start()
    {
        Controller.enabled = false;
    }

    public void OnConnectedr()
    {
        Controller.enabled = true;
        ConnectionCanvas.gameObject.SetActive(false);
    }

    // Update is called once per frame
    void Update()
    {
        if (ConnectionCanvas.gameObject.activeSelf)
        {
            var dots = Mathf.FloorToInt(Time.time % 3);
            ConnectingString.text = connectionString[dots];
        }
    }
}
