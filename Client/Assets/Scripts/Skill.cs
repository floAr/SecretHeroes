using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Skill : MonoBehaviour
{
    public string Name;
    public TMPro.TMP_Text Header;
    public TMPro.TMP_Text Value;

    private void OnValidate()
    {
        Header.text = Name;
    }
}
