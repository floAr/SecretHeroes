using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InstantiatedTokens : MonoBehaviour
{
    public static List<GameObject> instantiatedObjects;

    void Awake()
    {
        instantiatedObjects = new List<GameObject>();
    }
}
