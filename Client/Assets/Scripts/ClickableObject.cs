using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

public class ClickableObject : MonoBehaviour
{
    public Camera ObjectCamera;

    public UnityEvent OnClick;

    public void Start()
    {
        if (ObjectCamera == null)
        {
            Debug.LogError("Assign child camera to clickable Object!");
        }
        else
        {
            ObjectCamera.gameObject.SetActive(false);
        }
    }

    public Transform GetCameraPosition()
    {
        return ObjectCamera.transform;
    }
}