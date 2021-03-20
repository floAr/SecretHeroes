using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ClickableObject : MonoBehaviour
{
    public Camera ObjectCamera;

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