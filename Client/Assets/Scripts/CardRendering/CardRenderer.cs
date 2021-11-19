using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class CardRenderer : MonoBehaviour
{
    // Start is called before the first frame update
    public CardData Data;

    public GameObject[] Models;
    public Material[] Variants;
    private int _activeModel;

    // public Animator Animator;

    public string Name;
    public string Id;
    public TMPro.TMP_Text NameText;

    public Skill Weapons;
    public Skill Engineering;
    public Skill Biotech;
    public Skill Psychics;

    public Canvas SkillCanvas;

    public bool Hidden;

    public bool IsSelected;


    [Range(1, 100)]
    public int SkillWeapons;

    [Range(1, 100)]
    public int SkillEngineering;

    [Range(1, 100)]
    public int SkillBiotech;

    [Range(1, 100)]
    public int SkillPsychics;

    public int[] BaseSkills = new int[4];

    LTDescr _scaling;

    public void DoUpdate()
    {
        SetVisuals();
        SetSkills();
    }

    public void ReadToken(Token token)
    {
        Id = token.id;
        Name = token.name;
        NameText.text = Name;
        SkillWeapons = token.weapons;
        SkillEngineering = token.engineering;
        SkillBiotech = token.biotech;
        SkillPsychics = token.psychics;
        BaseSkills = new int[4] { token.base_weapons, token.base_engineering, token.base_biotech, token.base_psychics };
        DoUpdate();
    }

    public Token ToToken()
    {
        return new Token()
        {
            id = Id,
            name = Name,
            engineering = SkillEngineering,
            weapons = SkillWeapons,
            biotech = SkillBiotech,
            psychics = SkillPsychics,
            base_weapons = BaseSkills[0],
            base_engineering = BaseSkills[1],
            base_biotech = BaseSkills[2],
            base_psychics = BaseSkills[3]


        };
    }

    private void SetModel(int newModel)
    {
        for (int i = 0; i < Models.Length; i++)
        {
            Models[i].SetActive(false);
        }
        if (Hidden) return;
        Models[newModel].SetActive(true);
        _activeModel = newModel;
    }

    private void SetMaterial(int newVariant)
    {
        Models[_activeModel].GetComponent<SkinnedMeshRenderer>().material = Variants[newVariant];
    }


    private void Update()
    {
        // Animator.SetBool("IsSelected", IsSelected);
        SkillCanvas.gameObject.SetActive(IsSelected);
    }

    [ContextMenu("update")]
    private void SetVisuals()
    {

        int modelId = (BaseSkills[0] * 3 + BaseSkills[1] * 2 + BaseSkills[2] + BaseSkills[3]) % Models.Length;
        int materialId = (BaseSkills[0] + BaseSkills[1] + BaseSkills[2] * 3 + BaseSkills[3] * 2) % Variants.Length;

        SetModel(modelId);
        SetMaterial(materialId);
    }

    private void SetSkills()
    {
        Weapons.Value.text = SkillWeapons.ToString();
        Engineering.Value.text = SkillEngineering.ToString();
        Biotech.Value.text = SkillBiotech.ToString();
        Psychics.Value.text = SkillPsychics.ToString();
    }

    public void Reset()
    {
        // Animator.gameObject.transform.localPosition = Vector3.zero;
    }
}
